# Request ID Middleware

`with_request_id` assigns a unique UUID v4 to each incoming request via the `X-Request-Id` header. This request ID propagates to the response headers and is useful for request tracing and log correlation.

```rust
use fastrs::App;

#[tokio::main]
async fn main() {
    let app = App::new().with_request_id();
}
```

### Caveats & Notes
* If the incoming request already has an `X-Request-Id` header, that ID is preserved and propagated.
* For full logging benefits, combine this with the tracing middleware so the request ID is included in tracing spans.
