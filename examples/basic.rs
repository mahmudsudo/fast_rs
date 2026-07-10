use fastrs::{Json, OpenApi, Path, get, post};
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Validate, OpenApi)]
struct CreateUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 8))]
    password: String,
}

#[derive(Serialize, OpenApi)]
struct UserResponse {
    id: u64,
    email: String,
}

#[post("/users")]
async fn create_user(body: Json<CreateUser>) -> Json<UserResponse> {
    Json(UserResponse {
        id: 1,
        email: body.email.clone(),
    })
}

#[get("/users/{id}")]
async fn get_user(Path(id): Path<u64>) -> Json<UserResponse> {
    Json(UserResponse {
        id,
        email: "x@y.com".into(),
    })
}

fn main() {
    fastrs::App::new()
        .route(create_user)
        .route(get_user)
        .serve_docs_at("/docs")
        .run("0.0.0.0:8000");
}
