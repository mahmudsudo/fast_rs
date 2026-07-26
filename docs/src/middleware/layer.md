# App::layer() Escape Hatch

The `App::layer()` method is the foundational concept for middleware in `fastrs`. Every built-in middleware preset (such as CORS, tracing, rate limiting, and request IDs) is implemented internally as a wrapper over this escape hatch, which attaches Tower-compatible layers directly to the router.

```rust
use fastrs::{App, get};
use tower_http::compression::CompressionLayer;

#[get("/data")]
async fn data() -> &'static str {
    "compressed response"
}

#[tokio::main]
async fn main() {
    let app = App::new()
        .route(data())
        .layer(CompressionLayer::new()); // Attach any standard tower middleware
}
```

### Caveats & Notes
* Adding layers wraps all routes added BEFORE the layer is registered.
* Any Tower `Layer` that implements `Clone` and fits the request/response signature can be used.
