# CORS Middleware

`fastrs` supports CORS through the `with_cors` method. It is a convenience wrapper around `tower_http::cors::CorsLayer`.

```rust
use fastrs::App;
use tower_http::cors::{CorsLayer, Any};

#[tokio::main]
async fn main() {
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any);

    let app = App::new().with_cors(cors);
}
```

### Caveats & Notes
* Be careful with wildcards (`Any`) in production environments as they allow access from any domain.
* CORS headers are applied to both successfully processed requests and error responses.
