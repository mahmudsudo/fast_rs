//! HTMX integration via `axum-htmx` (optional `htmx` feature).
//!
//! Re-exports axum-htmx extractors/responders and adds OpenAPI header documentation.

pub use axum_htmx::{HxRedirect, HxRefresh, HxRequest, HxTarget, HxTrigger};

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
