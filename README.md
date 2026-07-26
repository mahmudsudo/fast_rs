# fastrs

[![Crates.io Version](https://img.shields.io/crates/v/fastrs-core.svg)](https://crates.io/crates/fastrs-core)
[![Crates.io Downloads](https://img.shields.io/crates/d/fastrs-core.svg)](https://crates.io/crates/fastrs-core)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![CI](https://github.com/mahmudsudo/fast_rs/actions/workflows/ci.yml/badge.svg)](https://github.com/mahmudsudo/fast_rs/actions)
[![Docs](https://img.shields.io/badge/docs-mdbook-informational)](https://mahmudsudo.github.io/fast_rs)

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

## Resources
- [Full documentation](https://mahmudsudo.github.io/fast_rs)
- [crates.io](https://crates.io/crates/fastrs-core)
- [Examples](./examples/)
- [CHANGELOG](./CHANGELOG.md)
