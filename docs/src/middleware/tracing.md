# Tracing Middleware

`with_tracing` integrates request-level tracing into your `fastrs` application. It wraps `tower_http::trace::TraceLayer` to automatically log HTTP requests and responses.

```rust
use fastrs::App;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    let app = App::new().with_tracing(TraceLayer::new_for_http());
}
```

### Caveats & Notes
* You must initialize a tracing subscriber (like `tracing_subscriber`) in your `main` function to see logs.
* Tracing levels and formats can be fully customized using `tower_http` config builders.
