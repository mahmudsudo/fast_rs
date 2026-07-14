# fastrs

A FastAPI-equivalent framework for Rust, built as a thin ergonomics layer on top of Axum. It delivers automatic request validation, auto-generated OpenAPI docs, and minimal boilerplate with zero runtime overhead.

## Quickstart

```rust
use fastrs::{get, post, Json, Path, OpenApi, Created};
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
async fn create_user(body: Json<CreateUser>) -> Created<Json<UserResponse>> {
    Created(Json(UserResponse { id: 1, email: body.email.clone() }))
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

## Features

- **Request validation** (via `validator` crate) with automatic 422 responses
- **Auto-generated OpenAPI/Swagger docs** at compile-time via macros
- **Extractors**: Query, Path, Json, Header, Page (pagination), Bearer (auth)
- **Response wrappers**: Created (201), NoContent (204), and more
- **Auth support**: Bearer token extractor with pluggable AuthVerifier trait
- **Pagination**: First-class Page extractor with default and custom bounds
- **Error handling**: Typed `ApiError` enum for automatic HTTP response mapping
- **Shared state**: `.with_state()` to wire databases, configs, or auth managers
- **Route nesting**: `.nest()` for clean API versioning (`/api/v1`, `/api/v2`)
- **Middleware**: `.with_cors()` and `.with_tracing()` for common middleware  
- **Zero lock-in**: Returns raw `axum::Router` – drop in, drop out

## Examples

- [**basic.rs**](examples/basic.rs) – Simple CRUD with validation
- [**todo_api.rs**](examples/todo_api.rs) – Full-featured Todo API with pagination, filtering, and custom response codes

## fastrs vs raw Axum vs FastAPI

| Feature | `fastrs` | `Axum` | `FastAPI` |
| --- | --- | --- | --- |
| **Boilerplate LOC** | Minimal (like FastAPI) | High (manual validation/schema) | Minimal |
| **OpenAPI Generation** | Compile-time via macros | None natively (3rd party: `utoipa`) | Runtime via reflection |
| **Request Validation** | Automatic, compile-time | Manual inside handler | Automatic |
| **Runtime Overhead** | Zero | Zero | High (Python) |
| **Routing Abstraction** | Returns raw `axum::Router` | Native | Custom |
| **Auth Support** | Bearer extractor + trait | Manual | OpenAPI auth schemes |
| **Pagination** | First-class `Page` extractor | Manual query parsing | Built-in |

## Architecture
See [docs/architecture.md](docs/architecture.md) for details on how `fastrs` expands macros into raw Axum abstractions with zero magic.
