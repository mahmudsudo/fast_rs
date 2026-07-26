# Request Validation

Request models implementing the `validator::Validate` trait are parsed and validated automatically when using the `Json<T>` extractor. If validation fails, `fastrs` halts execution and sends a structured error response back to the client.

```rust
use fastrs::{Json, post, OpenApi};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, OpenApi, Validate)]
struct RegisterUser {
    #[validate(length(min = 5))]
    password: String,
}

#[post("/register")]
async fn register(Json(data): Json<RegisterUser>) -> &'static str {
    "Registration successful"
}
```

### Caveats & Notes
* Validation relies entirely on the third-party `validator` crate.
* Custom validator functions must be registered carefully to integrate correctly.
