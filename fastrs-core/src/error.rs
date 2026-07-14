use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use validator::ValidationErrors;

#[derive(Debug, Serialize)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<FieldError>>,
}

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized(String),
    BadRequest(String),
    InternalServerError(String),
    Validation(ValidationErrors),
    Custom(StatusCode, String),
}

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
