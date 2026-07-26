# API Versioning

API versioning in `fastrs` is typically managed by nesting applications or using path prefixes during route registration. This structure isolates OpenAPI documentation for different API versions.

```rust
use fastrs::{App, get};

#[get("/users")]
async fn v1_users() -> &'static str { "v1" }

#[get("/users")]
async fn v2_users() -> &'static str { "v2" }

#[tokio::main]
async fn main() {
    let v1 = App::new().route(v1_users());
    let v2 = App::new().route(v2_users());
    let app = App::new().nest("/api/v1", v1).nest("/api/v2", v2);
}
```

### Caveats & Notes
* Nesting routes compiles down to axum sub-routing structure.
* The OpenAPI spec combines all paths under their fully nested endpoints.
