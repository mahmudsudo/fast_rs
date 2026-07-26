# Bearer<T> and AuthVerifier

`Bearer<T>` extracts the authorization token from the request headers and verifies it using the application state which must implement the `AuthVerifier<T>` trait. This enables a clean, type-safe authentication mechanism that integrates directly with dependency injection and OpenAPI.

```rust
use fastrs::{Bearer, AuthVerifier, ApiError, get, App, State};
use axum::async_trait;

#[derive(Clone)]
struct User {
    id: u64,
}

#[derive(Clone)]
struct AppState;

#[async_trait]
impl AuthVerifier<User> for AppState {
    type Error = ApiError;

    async fn verify(&self, token: &str) -> Result<User, Self::Error> {
        if token == "secret-token" {
            Ok(User { id: 42 })
        } else {
            Err(ApiError::Unauthorized("Invalid token".to_string()))
        }
    }
}

#[get("/secret")]
async fn secret_handler(Bearer(user): Bearer<User>) -> String {
    format!("Welcome user {}", user.id)
}
```

### Caveats & Notes
* The application state `S` must implement `AuthVerifier<T>` for whatever `T` is extracted.
* If the `Authorization` header is missing, malformed, or verification fails, a structured `ApiError::Unauthorized` or `ApiError::BadRequest` response is returned.
