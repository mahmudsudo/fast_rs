//! HTMX integration via `axum-htmx` (optional `htmx` feature).
//!
//! Re-exports axum-htmx extractors/responders and adds OpenAPI header documentation.

pub use axum_htmx::{HxRequest, HxTarget, HxTrigger};

use axum::{
    body::Body,
    http::HeaderValue,
    response::{IntoResponse, Response},
};

use crate::openapi::{OpenApiExtractor, Operation, Schema};

impl OpenApiExtractor for HxRequest {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: "HX-Request".to_string(),
            in_: "header".to_string(),
            required: false,
            schema: Schema {
                type_: Some("boolean".to_string()),
                ..Default::default()
            },
        });
    }
}

impl OpenApiExtractor for HxTarget {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: "HX-Target".to_string(),
            in_: "header".to_string(),
            required: false,
            schema: Schema {
                type_: Some("string".to_string()),
                ..Default::default()
            },
        });
    }
}

impl OpenApiExtractor for HxTrigger {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: "HX-Trigger".to_string(),
            in_: "header".to_string(),
            required: false,
            schema: Schema {
                type_: Some("string".to_string()),
                ..Default::default()
            },
        });
    }
}

/// Responder for returning an HTMX redirect response (`HX-Redirect` header).
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::HxRedirect;
///
/// async fn handler() -> HxRedirect {
///     HxRedirect("/target-page".to_string())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct HxRedirect(pub String);

impl IntoResponse for HxRedirect {
    fn into_response(self) -> Response {
        let mut response = Response::new(Body::empty());
        if let Ok(value) = HeaderValue::from_str(&self.0) {
            response.headers_mut().insert("HX-Redirect", value);
        }
        response
    }
}

impl crate::openapi::OpenApiResponder for HxRedirect {
    fn modify_operation(op: &mut Operation) {
        op.responses.insert(
            "302".to_string(),
            crate::openapi::Response {
                description: "HTMX redirect (HX-Redirect header)".to_string(),
                content: Default::default(),
            },
        );
    }
}

/// Responder for triggering a client-side page refresh via HTMX (`HX-Refresh` header).
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::HxRefresh;
///
/// async fn handler() -> HxRefresh {
///     HxRefresh(true)
/// }
/// ```
#[derive(Debug, Clone, Copy)]
pub struct HxRefresh(pub bool);


impl IntoResponse for HxRefresh {
    fn into_response(self) -> Response {
        let mut response = Response::new(Body::empty());
        let value = if self.0 { "true" } else { "false" };
        response
            .headers_mut()
            .insert("HX-Refresh", HeaderValue::from_static(value));
        response
    }
}

impl crate::openapi::OpenApiResponder for HxRefresh {
    fn modify_operation(op: &mut Operation) {
        op.responses.insert(
            "200".to_string(),
            crate::openapi::Response {
                description: "HTMX page refresh (HX-Refresh header)".to_string(),
                content: Default::default(),
            },
        );
    }
}
