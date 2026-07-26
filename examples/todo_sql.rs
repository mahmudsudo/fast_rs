use fastrs::{ApiError, App, Created, Json, NoContent, Page, Path, delete, get, post};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePool;

#[derive(Debug, Clone, Serialize, Deserialize, fastrs::OpenApi)]
struct TodoResponse {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Debug, Deserialize, validator::Validate, fastrs::OpenApi)]
struct CreateTodoRequest {
    #[validate(length(min = 1, message = "title must not be empty"))]
    title: String,
    #[serde(default)]
    done: bool,
}

/// Custom error wrapper pattern for database error mapping.
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

#[post("/api/sql/todos")]
async fn create_todo(
    axum::extract::State(pool): axum::extract::State<SqlitePool>,
    body: Json<CreateTodoRequest>,
) -> Result<Created<Json<TodoResponse>>, AppError> {
    let res = sqlx::query("INSERT INTO todos (title, done) VALUES (?, ?)")
        .bind(&body.title)
        .bind(body.done)
        .execute(&pool)
        .await?;

    let id = res.last_insert_rowid();

    let row: (i64, String, i64) = sqlx::query_as("SELECT id, title, done FROM todos WHERE id = ?")
        .bind(id)
        .fetch_one(&pool)
        .await?;

    Ok(Created(Json(TodoResponse {
        id: row.0,
        title: row.1,
        done: row.2 != 0,
    })))
}

#[get("/api/sql/todos")]
async fn list_todos(
    page: Page,
    axum::extract::State(pool): axum::extract::State<SqlitePool>,
) -> Result<Json<serde_json::Value>, AppError> {
    let offset = ((page.page - 1) * page.limit) as i64;
    let limit = page.limit as i64;

    let total_row: (i64,) = sqlx::query_as("SELECT COUNT(*) as count FROM todos")
        .fetch_one(&pool)
        .await?;

    let total = total_row.0;

    let rows: Vec<(i64, String, i64)> =
        sqlx::query_as("SELECT id, title, done FROM todos ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(limit)
            .bind(offset)
            .fetch_all(&pool)
            .await?;

    let items: Vec<TodoResponse> = rows
        .into_iter()
        .map(|(id, title, done)| TodoResponse {
            id,
            title,
            done: done != 0,
        })
        .collect();

    Ok(Json(serde_json::json!({
        "items": items,
        "total": total,
        "page": page.page,
        "limit": page.limit,
    })))
}

#[get("/api/sql/todos/{id}")]
async fn get_todo(
    Path(id): Path<i64>,
    axum::extract::State(pool): axum::extract::State<SqlitePool>,
) -> Result<Json<TodoResponse>, AppError> {
    let row: Option<(i64, String, i64)> =
        sqlx::query_as("SELECT id, title, done FROM todos WHERE id = ?")
            .bind(id)
            .fetch_optional(&pool)
            .await?;

    if let Some((row_id, title, done)) = row {
        Ok(Json(TodoResponse {
            id: row_id,
            title,
            done: done != 0,
        }))
    } else {
        Err(AppError(ApiError::NotFound(format!(
            "Todo {} not found",
            id
        ))))
    }
}

#[delete("/api/sql/todos/{id}")]
async fn delete_todo(
    Path(id): Path<i64>,
    axum::extract::State(pool): axum::extract::State<SqlitePool>,
) -> Result<NoContent, AppError> {
    let res = sqlx::query("DELETE FROM todos WHERE id = ?")
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
    // Use a shared in-memory SQLite DB so connections see the same data
    let database_url = "sqlite::memory:";
    let pool = SqlitePool::connect(&database_url)
        .await
        .expect("Failed to connect to sqlite");

    // Create table
    sqlx::query(
        r#"CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            title TEXT NOT NULL,
            done INTEGER NOT NULL DEFAULT 0
        )"#,
    )
    .execute(&pool)
    .await
    .expect("Failed to create table");

    let app = App::new()
        .route(create_todo)
        .route(list_todos)
        .route(get_todo)
        .route(delete_todo)
        .with_state(pool.clone())
        .serve_docs_at("/docs/sql");

    println!("SQL Todo example running on http://0.0.0.0:8001");
    println!("OpenAPI docs at http://0.0.0.0:8001/docs/sql");

    app.run("0.0.0.0:8001").await;
}
