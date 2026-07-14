use fastrs::axum::{
    async_trait,
    body::Body,
    http::{Request, StatusCode},
};
use fastrs::{App, AuthVerifier, Bearer, Json, OpenApi, Query, Created, NoContent, Page, Path, get, post, delete};
use serde::{Deserialize, Serialize};
use tower::ServiceExt;
use validator::Validate;

#[derive(Deserialize, Validate, OpenApi)]
struct CreateUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize, OpenApi)]
struct UserResponse {
    id: u64,
    email: String,
}

#[post("/users")]
async fn create_user(body: Json<CreateUser>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        email: body.email.clone(),
    })
}

#[derive(Deserialize, Validate, OpenApi)]
struct UserQuery {
    #[validate(range(min = 1))]
    page: u32,
}

#[get("/users")]
async fn list_users(query: Query<UserQuery>) -> Json<Vec<UserResponse>> {
    Json(vec![UserResponse {
        id: query.page as u64,
        email: "test@test.com".into(),
    }])
}

async fn body_bytes(body: Body) -> bytes::Bytes {
    use http_body_util::BodyExt;
    body.collect().await.unwrap().to_bytes()
}

#[tokio::test]
async fn test_valid_request() {
    let app = App::new().route(create_user).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email": "valid@email.com", "password": "password123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["id"], 1);
    assert_eq!(body["email"], "valid@email.com");
}

#[tokio::test]
async fn test_invalid_json_request_validation() {
    let app = App::new().route(create_user).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/users")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"email": "invalid", "password": "short"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    let errors = body["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 2);
    assert!(errors.iter().any(|e| e["field"] == "email"));
    assert!(errors.iter().any(|e| e["field"] == "password"));
}

#[tokio::test]
async fn test_query_validation() {
    let app = App::new().route(list_users).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/users?page=0") // invalid: min = 1
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);

    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();

    let errors = body["errors"].as_array().unwrap();
    assert_eq!(errors.len(), 1);
    assert_eq!(errors[0]["field"], "page");
}

#[tokio::test]
async fn test_route_nesting() {
    let sub_app = App::new().route(list_users);
    let app = App::new().nest("/api/v1", sub_app).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/api/v1/users?page=1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

// Tests for new v2 features

#[post("/items")]
async fn create_item(body: Json<CreateUser>) -> Created<Json<UserResponse>> {
    Created(Json(UserResponse {
        id: 1,
        email: body.email.clone(),
    }))
}

#[tokio::test]
async fn test_created_response_code() {
    let app = App::new().route(create_item).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/items")
                .header("content-type", "application/json")
                .body(Body::from(
                    r#"{"email": "valid@email.com", "password": "password123"}"#,
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["id"], 1);
}

#[delete("/items/{id}")]
async fn delete_item(Path(id): Path<u64>) -> NoContent {
    let _ = id;
    NoContent
}

#[tokio::test]
async fn test_no_content_response_code() {
    let app = App::new().route(delete_item).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri("/items/1")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
    let bytes = body_bytes(response.into_body()).await;
    assert!(bytes.is_empty());
}

#[get("/paginated")]
async fn get_paginated(page: Page) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "page": page.page,
        "limit": page.limit,
    }))
}

#[tokio::test]
async fn test_pagination_defaults() {
    let app = App::new().route(get_paginated).into_router();

    let response = app
        .clone()
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/paginated")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    
    assert_eq!(body["page"], 1);
    assert_eq!(body["limit"], 20);
}

#[tokio::test]
async fn test_pagination_custom() {
    let app = App::new().route(get_paginated).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/paginated?page=5&limit=50")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    
    assert_eq!(body["page"], 5);
    assert_eq!(body["limit"], 50);
}

#[tokio::test]
async fn test_pagination_bounds_validation() {
    let app = App::new().route(get_paginated).into_router();

    // Test page = 0 (invalid)
    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/paginated?page=0")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
}

#[derive(Clone)]
struct MockAuthVerifier;

#[async_trait]
impl AuthVerifier<String> for MockAuthVerifier {
    type Error = fastrs::ApiError;

    async fn verify(&self, token: &str) -> Result<String, Self::Error> {
        if token == "valid_token" {
            Ok("user123".to_string())
        } else {
            Err(fastrs::ApiError::Unauthorized("Invalid token".to_string()))
        }
    }
}

#[get("/protected")]
async fn protected_endpoint(Bearer(user): Bearer<String>) -> Json<serde_json::Value> {
    Json(serde_json::json!({ "user": user }))
}

#[tokio::test]
async fn test_bearer_auth_success() {
    let app = App::new()
        .with_state(MockAuthVerifier)
        .route(protected_endpoint)
        .into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .header("Authorization", "Bearer valid_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let bytes = body_bytes(response.into_body()).await;
    let body: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    assert_eq!(body["user"], "user123");
}

#[tokio::test]
async fn test_bearer_auth_missing_header() {
    let app = App::new()
        .with_state(MockAuthVerifier)
        .route(protected_endpoint)
        .into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_bearer_auth_invalid_token() {
    let app = App::new()
        .with_state(MockAuthVerifier)
        .route(protected_endpoint)
        .into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/protected")
                .header("Authorization", "Bearer invalid_token")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}
