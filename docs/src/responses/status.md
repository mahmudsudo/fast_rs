# Created<T> and NoContent

`Created<T>` forces an HTTP `201 Created` status code, and `NoContent` represents an empty body response mapping to HTTP `204 No Content`.

```rust
use fastrs::{Created, NoContent, post, delete, Json, OpenApi};
use serde::Serialize;

#[derive(Serialize, OpenApi)]
struct Resource {
    id: u64,
}

#[post("/resource")]
async fn create() -> Created<Json<Resource>> {
    Created(Json(Resource { id: 1 }))
}

#[delete("/resource")]
async fn delete_resource() -> NoContent {
    NoContent
}
```

### Caveats & Notes
* `Created<T>` propagates OpenAPI response updates, converting the default `200` response to `201`.
* `NoContent` removes the response body completely and updates the OpenAPI schema status to `204`.
