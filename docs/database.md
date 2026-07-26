# Database Integration Pattern

`fastrs` is designed to be completely unopinionated about database drivers, ORMs, or data access layers. Core `fastrs` does not include a hard dependency on any specific database crate (such as `sqlx`, `diesel`, or `sea-orm`), ensuring your binary stays lightweight and flexible.

---

## 1. Passing Database Pools via State

You share database pools (or connection state) across handlers using `.with_state()` on the `App` builder. Any type that implements `Clone + Send + Sync + 'static` (such as `sqlx::PgPool` or `sqlx::SqlitePool`) can be stored as application state.

```rust
use fastrs::App;
use sqlx::PgPool;

#[tokio::main]
async fn main() {
    let pool = PgPool::connect("postgres://postgres:postgres@localhost:5432/fastrs_todo").await.unwrap();

    let app = App::new()
        .route(create_todo)
        .route(list_todos)
        .with_state(pool);

    app.run("0.0.0.0:8000").await;
}
```

Handlers extract state using standard `axum::extract::State`:

```rust
use fastrs::{Json, post, Created};
use sqlx::PgPool;

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
```

---

## 2. Error Mapping via `From` Implementations

Because `fastrs` core does not depend on `sqlx` or any driver, database errors can be mapped to `ApiError` in your application binary. Define a local error wrapper struct `AppError` and implement `From<sqlx::Error>`:

```rust
use fastrs::ApiError;

#[derive(Debug)]
pub struct AppError(pub ApiError);

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        match err {
            sqlx::Error::RowNotFound => AppError(ApiError::NotFound("Resource not found".to_string())),
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
```

This clean conversion allows handler logic to use Rust's native `?` operator on database queries while returning `Result<T, AppError>`.

---

## 3. Example Codebases

- **SQLite Example**: [`examples/todo_sql.rs`](file:///home/mahmudsudo/fast_rs/examples/todo_sql.rs) demonstrates in-memory SQLite with `SqlitePool`.
- **PostgreSQL Example**: [`examples/todo-api-postgres/main.rs`](file:///home/mahmudsudo/fast_rs/examples/todo-api-postgres/main.rs) demonstrates PostgreSQL with `PgPool`, migrations, and full CRUD routes.
