# Json<T> Response

The `Json<T>` responder wraps serializeable payloads, automatically generating HTTP responses with the correct `application/json` Content-Type and compile-time OpenAPI response schemas.

```rust
use fastrs::{Json, get, OpenApi};
use serde::Serialize;

#[derive(Serialize, OpenApi)]
struct User {
    name: String,
}

#[get("/user")]
async fn get_user() -> Json<User> {
    Json(User { name: "Alice".to_string() })
}
```

### Caveats & Notes
* The inner type `T` must implement `Serialize` and `OpenApiType` for route schemas to resolve correctly.
* Any serialization failure results in an HTTP `500 Internal Server Error`.
