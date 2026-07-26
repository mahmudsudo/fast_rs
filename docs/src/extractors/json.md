# Json<T> Extractor

The `Json<T>` extractor deserializes JSON payloads from request bodies. It automatically integrates validation and updates the OpenAPI schema configuration.

```rust
use fastrs::{Json, post, OpenApi};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, OpenApi, Validate)]
struct CargoPayload {
    name: String,
}

#[post("/cargo")]
async fn create_cargo(Json(payload): Json<CargoPayload>) -> &'static str {
    "Success"
}
```

### Caveats & Notes
* If payload deserialization fails, an HTTP `400 Bad Request` or `422 Unprocessable Entity` is returned.
* Large payloads may need global size limitations configured at the middleware layer.
