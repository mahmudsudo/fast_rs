# Graceful Shutdown

`App::run` has built-in graceful shutdown support. It listens for `SIGINT` (Ctrl+C) and `SIGTERM` signals, allowing in-flight requests to complete before terminating the process.

```rust
use fastrs::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    
    // Starts the listener and registers shutdown signal handlers
    app.run("0.0.0.0:8000").await;
}
```

### Caveats & Notes
* If your application uses long-running WebSocket connections, you may need custom connection draining.
* Graceful shutdown requires the Tokio runtime is fully active and not blocked.
