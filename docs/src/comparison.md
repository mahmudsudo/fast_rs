# fastrs vs Axum vs FastAPI

`fastrs` offers a unique middle-ground, combining the speed of Rust's compile-time safety and type-safe systems with the high developer ergonomics of FastAPI.

```rust
// fastrs: compile-time validation & docs
#[post("/user")]
async fn create(Json(body): Json<User>) -> Created<Json<User>> {
    Created(Json(body))
}

// Axum: requires manual schema assembly and route mapping
// FastAPI: handles validation & docs but relies on runtime python interpreter overhead
```

### Caveats & Notes
* `fastrs` is built on top of Axum, meaning zero performance penalty for using its macro definitions.
* Unlike FastAPI, validation and OpenAPI schema reflection are performed at compile time, eliminating runtime reflection overhead.
