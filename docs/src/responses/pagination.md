# Pagination with Page

`fastrs` includes a built-in `Page` query extractor to automatically parse and validate request query parameters like `page` and `limit` with sensible defaults.

```rust
use fastrs::{Page, get, Json, OpenApi};
use serde::Serialize;

#[derive(Serialize, OpenApi)]
struct PaginatedUsers {
    items: Vec<String>,
    page: u32,
    limit: u32,
}

#[get("/users")]
async fn list_users(page: Page) -> Json<PaginatedUsers> {
    Json(PaginatedUsers {
        items: vec!["Alice".to_string(), "Bob".to_string()],
        page: page.page,
        limit: page.limit,
    })
}
```

### Caveats & Notes
* If `page` or `limit` are zero or negative, a `400 Bad Request` validation error is returned.
* The default query values are `page = 1` and `limit = 20`.
