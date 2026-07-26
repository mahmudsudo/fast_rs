#![cfg(feature = "htmx")]

use fastrs::axum::{
    body::Body,
    http::{Request, StatusCode},
};
use fastrs::{App, HxRedirect, HxRefresh, HxRequest, HxTarget, HxTrigger, OpenApi, get};
use tower::ServiceExt;

#[get("/htmx")]
async fn htmx_handler(
    HxRequest(is_htmx): HxRequest,
    HxTarget(target): HxTarget,
    HxTrigger(trigger): HxTrigger,
) -> fastrs::Json<serde_json::Value> {
    fastrs::Json(serde_json::json!({
        "htmx": is_htmx,
        "target": target,
        "trigger": trigger,
    }))
}

#[get("/htmx-refresh")]
async fn htmx_refresh() -> HxRefresh {
    HxRefresh(true)
}

#[get("/htmx-redirect")]
async fn htmx_redirect() -> HxRedirect {
    HxRedirect("/todos".to_string())
}

#[tokio::test]
async fn test_htmx_extractors_in_handler() {
    let app = App::new().route(htmx_handler).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/htmx")
                .header("HX-Request", "true")
                .header("HX-Target", "todo-list")
                .header("HX-Trigger", "refresh-btn")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    let body = http_body_util::BodyExt::collect(response.into_body())
        .await
        .unwrap()
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["htmx"], true);
    assert_eq!(json["target"], "todo-list");
    assert_eq!(json["trigger"], "refresh-btn");
}

#[tokio::test]
async fn test_htmx_openapi_schema_reflects_headers() {
    let app: App<()> = App::new().route(htmx_handler);
    let openapi = app.openapi;
    let op = openapi
        .paths
        .get("/htmx")
        .and_then(|p| p.get("get"))
        .expect("operation should exist");

    let names: Vec<_> = op.parameters.iter().map(|p| p.name.as_str()).collect();
    assert!(names.contains(&"HX-Request"));
    assert!(names.contains(&"HX-Target"));
    assert!(names.contains(&"HX-Trigger"));
}

#[tokio::test]
async fn test_htmx_refresh_responder() {
    let app = App::new().route(htmx_refresh).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/htmx-refresh")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("HX-Refresh").unwrap(), "true");
}

#[tokio::test]
async fn test_htmx_redirect_responder() {
    let app = App::new().route(htmx_redirect).into_router();

    let response = app
        .oneshot(
            Request::builder()
                .method("GET")
                .uri("/htmx-redirect")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    assert_eq!(response.headers().get("HX-Redirect").unwrap(), "/todos");
}
