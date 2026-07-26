# ApiError Response

`ApiError` is the standard error type in `fastrs` that represents common HTTP failures. It maps enum variants directly to HTTP status codes and structured JSON response payloads.

```rust
use fastrs::{ApiError, get};

#[get("/protected")]
async fn protected_route() -> Result<&'static str, ApiError> {
    Err(ApiError::Unauthorized("Access denied".to_string()))
}
```

### Caveats & Notes
* Validation errors automatically map to `422 Unprocessable Entity` containing field-level issues.
* Handlers returning `Result<T, ApiError>` must implement `IntoResponse` and `OpenApiResponder` on the success type `T`.
