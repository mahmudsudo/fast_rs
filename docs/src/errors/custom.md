# Custom Error Types

You can integrate custom application-specific errors with `fastrs` by implementing the `IntoApiError` trait. This allows automatic conversion of database or internal domain errors into HTTP responses.

```rust
use fastrs::{ApiError, IntoApiError, get};

#[derive(Debug)]
enum AppError {
    DatabaseConnectionFailed,
}

impl IntoApiError for AppError {
    fn into_api_error(self) -> ApiError {
        match self {
            AppError::DatabaseConnectionFailed => {
                ApiError::InternalServerError("Database is currently unreachable".to_string())
            }
        }
    }
}

#[get("/data")]
async fn data_handler() -> Result<&'static str, AppError> {
    Err(AppError::DatabaseConnectionFailed)
}
```

### Caveats & Notes
* Types implementing `IntoApiError` automatically gain a `From` implementation converting them into `ApiError`.
* Use `ApiError::Custom` if you need to return a custom HTTP status code with an arbitrary message.
