# fastrs

A FastAPI-equivalent framework for Rust, built as a thin ergonomics layer on top of Axum. It delivers automatic request validation, auto-generated OpenAPI docs, and minimal boilerplate with zero runtime overhead.

## Quickstart

```rust
use fastrs::{get, post, Json, Path, OpenApi};
use serde::{Deserialize, Serialize};
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
    Json(UserResponse { id: 1, email: body.email.clone() })
}

#[get("/users/{id}")]
async fn get_user(Path(id): Path<u64>) -> Json<UserResponse> {
    Json(UserResponse { id, email: "x@y.com".into() })
}

fn main() {
    fastrs::App::new()
        .route(create_user)
        .route(get_user)
        .serve_docs_at("/docs")
        .run("0.0.0.0:8000");
}
```

## fastrs vs raw Axum vs FastAPI

| Feature | `fastrs` | `Axum` | `FastAPI` |
| --- | --- | --- | --- |
| **Boilerplate LOC** | Minimal (like FastAPI) | High (Requires manual validation and schema wiring) | Minimal |
| **OpenAPI Generation** | Compile-time via macros | None natively, requires 3rd party (e.g., `utoipa`) | Runtime via reflection |
| **Request Validation** | Automatic, compile-time linked | Manual inside handler | Automatic |
| **Runtime Overhead** | Zero | Zero | High (Python reflection) |
| **Routing Abstraction** | Returns raw `axum::Router` | Native | Custom |

## Architecture
See `docs/architecture.md` for details on how `fastrs` expands macros into raw Axum abstractions with zero magic.
