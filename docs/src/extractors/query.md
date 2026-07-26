# Query<T> Extractor

`Query<T>` deserializes URL query parameters into a validated struct. The target struct must implement `Deserialize`, `OpenApi`, and `Validate`.

```rust
use fastrs::{Query, get, OpenApi};
use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, OpenApi, Validate)]
struct Pagination {
    limit: Option<usize>,
}

#[get("/items")]
async fn list_items(Query(pagination): Query<Pagination>) -> &'static str {
    "Items list"
}
```

### Caveats & Notes
* Missing query parameters are initialized as `None` if they are wrapped in `Option`.
* Incorrectly formatted query parameters cause a deserialization error response.
