use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    response::{IntoResponse, Response},
};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::BTreeMap;
use validator::Validate;

use crate::openapi::{MediaType, OpenApiExtractor, OpenApiResponder, OpenApiType, Operation, RequestBody};

pub struct Json<T>(pub T);

impl<T> std::ops::Deref for Json<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

#[axum::async_trait]
impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let axum::extract::Json(value) = axum::extract::Json::<T>::from_request(req, state)
            .await
            .map_err(|e| e.into_response())?;

        value.validate().map_err(|e| {
            (
                axum::http::StatusCode::UNPROCESSABLE_ENTITY,
                axum::Json(e),
            )
                .into_response()
        })?;

        Ok(Json(value))
    }
}

impl<T: OpenApiType> OpenApiExtractor for Json<T> {
    fn modify_operation(op: &mut Operation) {
        let mut content = BTreeMap::new();
        content.insert(
            "application/json".to_string(),
            MediaType {
                schema: T::schema(),
            },
        );
        op.request_body = Some(RequestBody {
            content,
            required: true,
        });
    }
}

impl<T: OpenApiType> OpenApiResponder for Json<T> {
    fn modify_operation(op: &mut Operation) {
        let mut content = BTreeMap::new();
        content.insert(
            "application/json".to_string(),
            MediaType {
                schema: T::schema(),
            },
        );
        op.responses.insert(
            "200".to_string(),
            crate::openapi::Response {
                description: "Successful response".to_string(),
                content,
            },
        );
    }
}

pub struct Path<T>(pub T);

impl<T> std::ops::Deref for Path<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Path<T>
where
    T: DeserializeOwned + Send + Sync,
    S: Send + Sync,
{
    type Rejection = axum::extract::rejection::PathRejection;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::Path(value) =
            axum::extract::Path::<T>::from_request_parts(parts, state).await?;
        Ok(Path(value))
    }
}
