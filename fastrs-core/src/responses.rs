use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use std::collections::BTreeMap;

use crate::openapi::{OpenApiResponder, Response as OpenApiResponse};

pub struct Created<T>(pub T);

impl<T> std::ops::Deref for Created<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: IntoResponse> IntoResponse for Created<T> {
    fn into_response(self) -> Response {
        let response = self.0.into_response();
        let (mut parts, body) = response.into_parts();
        parts.status = StatusCode::CREATED;
        Response::from_parts(parts, body)
    }
}

impl<T: OpenApiResponder> OpenApiResponder for Created<T> {
    fn modify_operation(op: &mut crate::openapi::Operation) {
        let mut temp_op = crate::openapi::Operation::default();
        T::modify_operation(&mut temp_op);

        for (status, response) in temp_op.responses {
            let target_status = if status == "200" {
                "201".to_string()
            } else {
                status
            };
            op.responses.insert(target_status, response);
        }
    }
}

pub struct NoContent;

impl IntoResponse for NoContent {
    fn into_response(self) -> Response {
        (StatusCode::NO_CONTENT, "").into_response()
    }
}

impl OpenApiResponder for NoContent {
    fn modify_operation(op: &mut crate::openapi::Operation) {
        op.responses.insert(
            "204".to_string(),
            OpenApiResponse {
                description: "No Content".to_string(),
                content: BTreeMap::new(),
            },
        );
    }
}
