# Path<T> Extractor

`Path<T>` extracts segments from the URL path. It validates and registers the parameters inside the OpenAPI schema automatically.

```rust
use fastrs::{Path, get};

#[get("/users/{id}")]
async fn get_user(Path(id): Path<u64>) -> String {
    format!("User ID: {}", id)
}
```

### Caveats & Notes
* Variables in the route path (e.g. `{id}`) must match the parameter name in the handler.
* Parsing failures result in immediate client-side bad request errors.
