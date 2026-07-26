# Introduction

`fastrs` is a high-performance web framework for Rust designed to provide a FastAPI-like developer experience. It achieves compile-time OpenAPI schema generation, request validation, and clean routing using powerful macro features.

```rust
use fastrs::{App, get};

#[get("/hello")]
async fn hello() -> &'static str {
    "Hello, World!"
}

#[tokio::main]
async fn main() {
    let app = App::new().route(hello());
    app.run("127.0.0.1:3000").await.unwrap();
}
```

### Caveats & Notes
* All route definitions are resolved and registered at compile time.
* The API endpoints automatically populate the OpenAPI registry for automated interactive documentation (Swagger UI).
