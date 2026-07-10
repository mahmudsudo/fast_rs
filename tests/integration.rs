use fastrs::axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fastrs::{App, Json, OpenApi, Query, get, post};
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
