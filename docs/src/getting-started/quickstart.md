# Quickstart

A minimal `fastrs` application handles request validation, routes definition, and starts an HTTP server serving interactive OpenAPI documentation. You define your request model using the `OpenApi` and `Validate` derive macros.

```rust
use fastrs::{App, post, Json, Created, OpenApi};
use validator::Validate;
use serde::Deserialize;

#[derive(Deserialize, OpenApi, Validate)]
struct CreateUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 3))]
    username: String,
}

#[post("/users")]
async fn create_user(Json(body): Json<CreateUser>) -> Created<Json<CreateUser>> {
    Created(Json(body))
}

#[tokio::main]
async fn main() {
    let app = App::new().route(create_user());
    app.run("127.0.0.1:3000").await.unwrap();
}
```

### Caveats & Notes
* Request validation occurs before your handler function is executed.
* Invalid payloads automatically trigger structured validation errors (HTTP 400).
