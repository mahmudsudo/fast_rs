use fastrs::{ApiError, App, Created, Json, NoContent, Page, Path, delete, get, patch, post};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, fastrs::OpenApi, sqlx::FromRow)]
pub struct TodoResponse {
    pub id: i64,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Serialize, fastrs::OpenApi)]
pub struct TodoListResponse {
    pub items: Vec<TodoResponse>,
    pub total: i64,
    pub page: u32,
    pub limit: u32,
}

#[derive(Debug, Deserialize, Validate, fastrs::OpenApi)]
pub struct CreateTodoRequest {
    #[validate(length(min = 1, message = "title must not be empty"))]
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[derive(Debug, Deserialize, Validate, fastrs::OpenApi)]
pub struct UpdateTodoRequest {
    pub title: Option<String>,
    pub done: Option<bool>,
}

/// Custom error wrapper pattern for database error mapping.
/// Since `ApiError` and `sqlx::Error` are defined in external crates,
/// binary applications wrap or convert `sqlx::Error` into `AppError` (or `ApiError`).
#[derive(Debug)]
pub struct AppError(pub ApiError);

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError(ApiError::NotFound("Todo not found".to_string())),
            _ => AppError(ApiError::InternalServerError(err.to_string())),
        }
    }
}

impl From<ApiError> for AppError {
    fn from(err: ApiError) -> Self {
        AppError(err)
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        self.0.into_response()
    }
}

impl fastrs::OpenApiResponder for AppError {
    fn modify_operation(op: &mut fastrs::Operation) {
        ApiError::modify_operation(op);
    }
}

#[post("/api/v1/todos")]
async fn create_todo(
    axum::extract::State(pool): axum::extract::State<PgPool>,
    body: Json<CreateTodoRequest>,
) -> Result<Created<Json<TodoResponse>>, AppError> {
    let row = sqlx::query_as::<_, TodoResponse>(
        "INSERT INTO todos (title, done) VALUES ($1, $2) RETURNING id, title, done",
    )
    .bind(&body.title)
    .bind(body.done)
    .fetch_one(&pool)
    .await?;

    Ok(Created(Json(row)))
}

#[get("/api/v1/todos")]
async fn list_todos(
    page: Page,
    axum::extract::State(pool): axum::extract::State<PgPool>,
) -> Result<Json<TodoListResponse>, AppError> {
    let offset = ((page.page - 1) * page.limit) as i64;
    let limit = page.limit as i64;

    let (total,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM todos")
        .fetch_one(&pool)
        .await?;

    let items = sqlx::query_as::<_, TodoResponse>(
        "SELECT id, title, done FROM todos ORDER BY id DESC LIMIT $1 OFFSET $2",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&pool)
    .await?;

    Ok(Json(TodoListResponse {
        items,
        total,
        page: page.page,
        limit: page.limit,
    }))
}

#[get("/api/v1/todos/{id}")]
async fn get_todo(
    Path(id): Path<i64>,
    axum::extract::State(pool): axum::extract::State<PgPool>,
) -> Result<Json<TodoResponse>, AppError> {
    let todo = sqlx::query_as::<_, TodoResponse>("SELECT id, title, done FROM todos WHERE id = $1")
        .bind(id)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| ApiError::NotFound(format!("Todo {} not found", id)))?;

    Ok(Json(todo))
}

#[patch("/api/v1/todos/{id}")]
async fn update_todo(
    Path(id): Path<i64>,
    axum::extract::State(pool): axum::extract::State<PgPool>,
    body: Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, AppError> {
    let existing =
        sqlx::query_as::<_, TodoResponse>("SELECT id, title, done FROM todos WHERE id = $1")
            .bind(id)
            .fetch_optional(&pool)
            .await?
            .ok_or_else(|| ApiError::NotFound(format!("Todo {} not found", id)))?;

    let new_title = body.title.clone().unwrap_or(existing.title);
    let new_done = body.done.unwrap_or(existing.done);

    let updated = sqlx::query_as::<_, TodoResponse>(
        "UPDATE todos SET title = $1, done = $2 WHERE id = $3 RETURNING id, title, done",
    )
    .bind(new_title)
    .bind(new_done)
    .bind(id)
    .fetch_one(&pool)
    .await?;

    Ok(Json(updated))
}

#[delete("/api/v1/todos/{id}")]
async fn delete_todo(
    Path(id): Path<i64>,
    axum::extract::State(pool): axum::extract::State<PgPool>,
) -> Result<NoContent, AppError> {
    let res = sqlx::query("DELETE FROM todos WHERE id = $1")
        .bind(id)
        .execute(&pool)
        .await?;

    if res.rows_affected() == 0 {
        return Err(AppError(ApiError::NotFound(format!(
            "Todo {} not found",
            id
        ))));
    }

    Ok(NoContent)
}

#[tokio::main]
async fn main() {
    let db_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgres://postgres:postgres@localhost:5432/fastrs_todo".to_string());

    let pool = PgPool::connect(&db_url).await.expect(
        "Failed to connect to Postgres. Make sure DATABASE_URL is set and Postgres is running.",
    );

    let app = App::new()
        .route(create_todo)
        .route(list_todos)
        .route(get_todo)
        .route(update_todo)
        .route(delete_todo)
        .with_state(pool)
        .serve_docs_at("/docs");

    println!("Postgres Todo example running on http://0.0.0.0:8002");
    println!("OpenAPI docs available at http://0.0.0.0:8002/docs");

    app.run("0.0.0.0:8002").await;
}
