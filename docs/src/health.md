# Health Checks

`fastrs` provides built-in methods on `App` to expose basic or custom health check endpoints, returning `200 OK` or `503 Service Unavailable`.

```rust
use fastrs::App;

#[tokio::main]
async fn main() {
    let app = App::new()
        // Simple health check endpoint returning {"status": "ok", "version": "..."}
        .health_check("/health")
        // Custom health check checking database connectivity
        .health_check_with("/health/db", || async {
            Ok(()) // or Err("database unreachable".to_string())
        });
}
```

### Caveats & Notes
* The check function provided to `health_check_with` must be thread-safe and return a `Result<(), String>`.
* Health check routes are excluded from the auto-generated OpenAPI documentation to avoid noise.
