# fastrs Architecture

`fastrs` is designed as a thin ergonomics layer over Axum. It provides the developer experience of FastAPI—automatic OpenAPI generation and request validation—without introducing runtime overhead or custom routing abstractions.

## How it works

### 1. `fastrs-core`
The core runtime library provides thin wrappers around standard Axum extractors:
- `fastrs::Json<T>`: Wraps `axum::extract::Json` and implements `FromRequest` requiring `T: validator::Validate`. This automatically validates payloads before the handler runs, returning a `422 Unprocessable Entity` with JSON errors if validation fails.
- `fastrs::Path<T>`: Wraps `axum::extract::Path`.

It also provides the `fastrs::App` struct, which wraps `axum::Router` and automatically aggregates an OpenAPI schema as routes are registered. The `into_router()` method allows immediate fallback to the raw Axum router.

### 2. `fastrs-macros`
`fastrs` heavily utilizes proc-macros for compile-time generation.

#### `#[derive(OpenApi)]`
Generates an implementation of the `OpenApiType` trait. It parses the struct fields and their types to generate a static schema. Crucially, it inspects `#[validate(...)]` attributes (from the `validator` crate) to inject constraints like `format: "email"` or `min_length` directly into the generated OpenAPI schema.

#### Routing Macros (`#[get]`, `#[post]`, etc.)
When a handler is annotated with a routing macro:

```rust
#[post("/users")]
async fn create_user(body: Json<CreateUser>) -> Json<UserResponse> { ... }
```

The macro transforms it into a factory function returning a `RouteDef`:
```rust
#[allow(non_camel_case_types)]
pub fn create_user() -> fastrs::RouteDef {
    async fn __fastrs_inner_create_user(body: Json<CreateUser>) -> Json<UserResponse> { ... }

    let mut op = fastrs::Operation::default();
    <Json<CreateUser> as fastrs::OpenApiExtractor>::modify_operation(&mut op);
    <Json<UserResponse> as fastrs::OpenApiResponder>::modify_operation(&mut op);

    fastrs::RouteDef {
        path: "/users",
        method: fastrs::Method::Post,
        router: axum::routing::post(__fastrs_inner_create_user),
        operation: op,
    }
}
```

This factory function is passed to `fastrs::App::new().route(...)`, which uses the `RouteDef` to register the raw Axum handler while capturing the operation details for the OpenAPI schema.
