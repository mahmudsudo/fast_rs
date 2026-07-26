# DB Integration Pattern

Connecting to a relational database like PostgreSQL or SQLite is typically done by embedding the database pool directly in the shared application state struct.

```rust
use fastrs::{App, get, State};
use sqlx::{SqlitePool, query};

#[derive(Clone)]
struct AppState {
    pool: SqlitePool,
}

#[get("/users/count")]
async fn count_users(State(state): State<AppState>) -> String {
    let count: (i64,) = query_as("SELECT COUNT(*) FROM users")
        .fetch_one(&state.pool)
        .await
        .unwrap();
    format!("Total users: {}", count.0)
}

// Dummy helper for example compilation
fn query_as(sql: &str) -> sqlx::query::Map<'static, sqlx::Sqlite, fn(sqlx::sqlite::SqliteRow) -> Result<(i64,), sqlx::Error>, sqlx::sqlite::SqliteArguments<'static>> {
    sqlx::query_as(sql)
}
```

### Caveats & Notes
* Always manage pool lifetimes by passing the pool inside an Arc or using natively cloneable pool structures like `sqlx::Pool`.
* Run migrations before starting the `fastrs` application server.
