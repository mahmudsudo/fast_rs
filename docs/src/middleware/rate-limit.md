# Rate Limiting Middleware

`with_rate_limit` limits incoming request frequency based on a sliding window. It rejects excessive requests immediately with an HTTP `429 Too Many Requests` status code.

```rust
use fastrs::{App, RateLimitConfig};
use std::time::Duration;

#[tokio::main]
async fn main() {
    let limit_config = RateLimitConfig::new(60, Duration::from_secs(60));
    let app = App::new().with_rate_limit(limit_config);
}
```

### Caveats & Notes
* This middleware currently uses an in-memory sliding window store (`rate_rs`).
* In-memory storage is not synchronized across multiple instances behind a load balancer.
