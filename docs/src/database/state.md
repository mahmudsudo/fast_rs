# with_state() Pattern

In `fastrs`, application state is shared across handlers using the `with_state()` method. This initializes the router's state context, converting the app from type `App<S>` to `App<()>`.

```rust
use fastrs::{App, get, State};

#[derive(Clone)]
struct DatabaseConnection {
    url: String,
}

#[get("/db")]
async fn db_handler(State(db): State<DatabaseConnection>) -> String {
    format!("Connecting to {}", db.url)
}

#[tokio::main]
async fn main() {
    let state = DatabaseConnection { url: "postgres://localhost/db".to_string() };
    let app = App::new()
        .route(db_handler())
        .with_state(state);

    app.run("127.0.0.1:3000").await.unwrap();
}
```

### Caveats & Notes
* Application state must implement the `Clone` trait.
* After calling `with_state()`, you cannot register additional routes that depend on state `S`.
