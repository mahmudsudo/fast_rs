use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

/// Represents a single validation error on a request field.
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::error::FieldError;
///
/// let error = FieldError {
///     field: "email".to_string(),
///     message: "Invalid email format".to_string(),
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

/// The structured JSON payload returned on API errors.
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::error::ErrorResponse;
///
/// let resp = ErrorResponse {
///     message: Some("An error occurred".to_string()),
///     errors: None,
/// };
/// ```
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

/// Common API error variants that map directly to HTTP status codes.
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::ApiError;
///
/// let error = ApiError::NotFound("User not found".to_string());
/// ```
#[derive(Debug)]
pub enum ApiError {
    /// 404 Not Found error.
    NotFound(String),
    /// 401 Unauthorized error.
    Unauthorized(String),
    /// 400 Bad Request error.
    BadRequest(String),
    /// 500 Internal Server Error.
    InternalServerError(String),
    /// 422 Unprocessable Entity error wrapping validation errors.
    Validation(ValidationErrors),
    /// Arbitrary status code custom error.
    Custom(StatusCode, String),
}

/// A trait for converting application-specific error types into `ApiError`.
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::{ApiError, IntoApiError};
///
/// struct MyDbError;
///
/// impl IntoApiError for MyDbError {
///     fn into_api_error(self) -> ApiError {
///         ApiError::InternalServerError("Database failure".to_string())
///     }
/// }
/// ```
pub trait IntoApiError {
    fn into_api_error(self) -> ApiError;
}

impl<T: IntoApiError> From<T> for ApiError {
    fn from(err: T) -> Self {
        err.into_api_error()
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status, body) = match self {
            ApiError::NotFound(msg) => (
                StatusCode::NOT_FOUND,
                ErrorResponse {
                    message: Some(msg),
                    errors: None,
                },
            ),
            ApiError::BadRequest(msg) => (
                StatusCode::BAD_REQUEST,
                ErrorResponse {
                    message: Some(msg),
                    errors: None,
                },
            ),
            ApiError::Unauthorized(msg) => (
                StatusCode::UNAUTHORIZED,
                ErrorResponse {
                    message: Some(msg),
                    errors: None,
                },
            ),
            ApiError::InternalServerError(msg) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                ErrorResponse {
                    message: Some(msg),
                    errors: None,
                },
            ),
            ApiError::Custom(status, msg) => (
                status,
                ErrorResponse {
                    message: Some(msg),
                    errors: None,
                },
            ),
            ApiError::Validation(errors) => {
                let mut field_errors = Vec::new();
                for (field, errs) in errors.field_errors() {
                    for err in errs {
                        let msg = err
                            .message
                            .as_ref()
                            .map(|cow| cow.to_string())
                            .unwrap_or_else(|| format!("validation failed: {}", err.code));
                        field_errors.push(FieldError {
                            field: field.to_string(),
                            message: msg,
                        });
                    }
                }
                (
                    StatusCode::UNPROCESSABLE_ENTITY,
                    ErrorResponse {
                        message: None,
                        errors: Some(field_errors),
                    },
                )
            }
        };

        (status, Json(body)).into_response()
    }
}

impl crate::openapi::OpenApiResponder for ApiError {
    fn modify_operation(op: &mut crate::openapi::Operation) {
        let mut content = std::collections::BTreeMap::new();

        let mut props = std::collections::BTreeMap::new();
        props.insert(
            "message".to_string(),
            crate::openapi::Schema {
                type_: Some("string".into()),
                ..Default::default()
            },
        );

        let schema = crate::openapi::Schema {
            type_: Some("object".into()),
            properties: props,
            ..Default::default()
        };

        content.insert(
            "application/json".to_string(),
            crate::openapi::MediaType { schema },
        );

        op.responses.insert(
            "4XX".to_string(),
            crate::openapi::Response {
                description: "Client Error".into(),
                content: content.clone(),
            },
        );
        op.responses.insert(
            "5XX".to_string(),
            crate::openapi::Response {
                description: "Server Error".into(),
                content,
            },
        );
    }
}

impl<T: crate::openapi::OpenApiResponder, E: crate::openapi::OpenApiResponder>
    crate::openapi::OpenApiResponder for Result<T, E>
{
    fn modify_operation(op: &mut crate::openapi::Operation) {
        T::modify_operation(op);
        E::modify_operation(op);
    }
}
