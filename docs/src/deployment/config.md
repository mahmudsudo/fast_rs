# Environment Configuration

Configure production parameters (like ports, host addresses, database connections) using environment variables. You can parse them during application startup.

```rust
use fastrs::App;
use std::env;

#[tokio::main]
async fn main() {
    let port = env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    let host = env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let addr = format!("{}:{}", host, port);

    let app = App::new();
    app.run(&addr).await;
}
```

### Caveats & Notes
* Missing environment variables should have safe, documented fallback values.
* Always validate parsed integers or configurations at startup to crash early if invalid.
