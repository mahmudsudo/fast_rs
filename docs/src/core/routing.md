# How Routing Macros Work

`fastrs` implements routing using procedural attribute macros (`#[get]`, `#[post]`, etc.) to intercept the handler signature and automatically generate OpenAPI operations. These macros output a `RouteDef<S>` struct containing paths, HTTP methods, and extracted parameters.

```rust
use fastrs::{get, RouteDef};

#[get("/items/{id}")]
async fn get_item(id: String) -> &'static str {
    "item"
}

// Under the hood, this expands to:
// pub fn get_item<S>() -> RouteDef<S> { ... }
```

### Caveats & Notes
* The router uses `axum::routing` internally to dispatch incoming requests.
* Handlers must return types that implement the `IntoResponse` and `OpenApiResponder` traits.
