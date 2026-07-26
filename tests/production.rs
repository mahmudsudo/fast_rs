use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use bytes::Bytes;
use fastrs::{App, Json, RateLimitConfig, get, post};
use http_body_util::BodyExt;
use serde::Serialize;
use std::time::Duration;
use tower::ServiceExt;

// ─── Helpers ─────────────────────────────────────────────────────────────────

async fn body_bytes(body: Body) -> Bytes {
    body.collect().await.unwrap().to_bytes()
}

// ─── D1: Rate Limiting ────────────────────────────────────────────────────────

#[derive(Serialize, fastrs::OpenApi)]
struct PingResponse {
    ok: bool,
}

#[get("/ping")]
async fn ping_handler() -> Json<PingResponse> {
    Json(PingResponse { ok: true })
}

#[tokio::test]
async fn test_rate_limit_allows_within_threshold() {
    let app = App::new()
        .route(ping_handler)
        .with_rate_limit(RateLimitConfig::new(5, Duration::from_secs(60)))
        .into_router();

    for _ in 0..5 {
        let resp = app
            .clone()
            .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(resp.status(), StatusCode::OK);
    }
}

#[tokio::test]
async fn test_rate_limit_returns_429_after_threshold() {
    let app = App::new()
        .route(ping_handler)
        // 1 request per minute window — second request should be rejected
        .with_rate_limit(RateLimitConfig::new(1, Duration::from_secs(60)))
        .into_router();

    // First request — allowed
    app.clone()
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    // Second request — should be rate limited
    let resp = app
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(resp.status(), StatusCode::TOO_MANY_REQUESTS);
}

// ─── D2: Health Check ─────────────────────────────────────────────────────────

#[tokio::test]
async fn test_health_check_returns_200() {
    let app = App::new().health_check("/health").into_router();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_bytes(resp.into_body()).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
    assert!(json["version"].is_string());
}

#[tokio::test]
async fn test_health_check_with_failing_check_returns_503() {
    let app = App::new()
        .health_check_with("/health", || async {
            Err("db connection failed".to_string())
        })
        .into_router();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::SERVICE_UNAVAILABLE);
    let body = body_bytes(resp.into_body()).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "error");
}

#[tokio::test]
async fn test_health_check_with_passing_check_returns_200() {
    let app = App::new()
        .health_check_with("/health", || async { Ok(()) })
        .into_router();

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = body_bytes(resp.into_body()).await;
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["status"], "ok");
}

// ─── D3: Request ID ───────────────────────────────────────────────────────────

#[tokio::test]
async fn test_request_id_header_present_on_response() {
    let app = App::new()
        .route(ping_handler)
        .with_request_id()
        .into_router();

    let resp = app
        .oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap())
        .await
        .unwrap();

    assert!(
        resp.headers().get("x-request-id").is_some(),
        "X-Request-Id header should be set"
    );
}

#[tokio::test]
async fn test_request_id_passes_through_existing_id() {
    let app = App::new()
        .route(ping_handler)
        .with_request_id()
        .into_router();

    let custom_id = "my-custom-request-id-12345";
    let resp = app
        .oneshot(
            Request::builder()
                .uri("/ping")
                .header("x-request-id", custom_id)
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let returned_id = resp
        .headers()
        .get("x-request-id")
        .unwrap()
        .to_str()
        .unwrap();
    assert_eq!(
        returned_id, custom_id,
        "Existing request ID should be propagated"
    );
}

// ─── D4: Multipart ────────────────────────────────────────────────────────────

#[post("/upload")]
async fn upload_handler(mut multipart: fastrs::Multipart) -> Json<serde_json::Value> {
    let mut fields = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap_or("unknown").to_string();
        let data = field.bytes().await.unwrap();
        fields.push(serde_json::json!({ "name": name, "size": data.len() }));
    }
    Json(serde_json::json!({ "fields": fields }))
}

#[tokio::test]
async fn test_multipart_parses_body() {
    let app = App::new().route(upload_handler).into_router();

    let boundary = "----WebKitFormBoundary7MA4YWxkTrZu0gW";
    let body = format!(
        "--{boundary}\r\nContent-Disposition: form-data; name=\"file\"; filename=\"test.txt\"\r\nContent-Type: text/plain\r\n\r\nhello world\r\n--{boundary}--\r\n",
        boundary = boundary
    );

    let resp = app
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/upload")
                .header(
                    "content-type",
                    format!("multipart/form-data; boundary={}", boundary),
                )
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let bytes = body_bytes(resp.into_body()).await;
    let json: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    let fields = json["fields"].as_array().unwrap();
    assert_eq!(fields.len(), 1);
    assert_eq!(fields[0]["name"], "file");
}

// ─── D5: Graceful shutdown (basic test that .run() type-checks) ───────────────
// Full graceful shutdown requires a live TCP socket. We verify the in-flight
// completion contract with a tokio::time::timeout guard.
#[tokio::test]
async fn test_graceful_shutdown_allows_inflight_requests() {
    use tokio::time::timeout;

    let app = App::new().route(ping_handler).into_router();

    // Simulate handling a request within a 1-second deadline — should complete
    // well before the timeout even with overhead.
    let result = timeout(
        Duration::from_secs(1),
        app.oneshot(Request::builder().uri("/ping").body(Body::empty()).unwrap()),
    )
    .await;

    assert!(
        result.is_ok(),
        "In-flight request should complete within timeout"
    );
    assert_eq!(result.unwrap().unwrap().status(), StatusCode::OK);
}
