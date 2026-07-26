# Postgres Todo API Example

This example demonstrates how to integrate PostgreSQL with `fastrs` using `sqlx`. `fastrs` stays unopinionated about database drivers or ORMs, allowing you to pass any thread-safe pool (like `sqlx::PgPool`) to your application state via `.with_state(pool)`.

---

## Setup & Running

### 1. Set `DATABASE_URL`
```bash
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/fastrs_todo"
```

### 2. Run Migrations
```bash
sqlx db create
sqlx migrate run --source examples/todo-api-postgres/migrations
```

### 3. Run the Example
```bash
cargo run --example todo-api-postgres
```

The server starts on `http://0.0.0.0:8002` with OpenAPI docs at `http://0.0.0.0:8002/docs`.

---

## cURL Examples for All 5 Routes

### 1. Create a Todo (`POST /api/v1/todos`)
```bash
curl -X POST http://localhost:8002/api/v1/todos \
  -H "Content-Type: application/json" \
  -d '{"title": "Buy groceries", "done": false}'
```

### 2. List Todos Paginated (`GET /api/v1/todos`)
```bash
curl "http://localhost:8002/api/v1/todos?page=1&limit=10"
```

### 3. Get Todo by ID (`GET /api/v1/todos/{id}`)
```bash
curl http://localhost:8002/api/v1/todos/1
```

### 4. Update Todo (`PATCH /api/v1/todos/{id}`)
```bash
curl -X PATCH http://localhost:8002/api/v1/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"done": true}'
```

### 5. Delete Todo (`DELETE /api/v1/todos/{id}`)
```bash
curl -X DELETE http://localhost:8002/api/v1/todos/1
```

---

## Database Error Mapping Pattern (Copy This Pattern)

Because `fastrs` core does not include a hard dependency on any specific database driver, you map `sqlx::Error` to `ApiError` in your application binary using a wrapper type or conversion function:

```rust
// COPY THIS PATTERN into your application code
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

With this pattern, all handlers using `sqlx` can use the `?` operator directly on database queries while returning `Result<T, AppError>`.
