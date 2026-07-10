pub use axum::extract::State;
use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeMap;
use validator::Validate;

use crate::openapi::{
    MediaType, OpenApiExtractor, OpenApiResponder, OpenApiType, Operation, RequestBody,
};

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

        value
            .validate()
            .map_err(|e| crate::error::ApiError::Validation(e).into_response())?;

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

pub struct Query<T>(pub T);

impl<T> std::ops::Deref for Query<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Query<T>
where
    T: DeserializeOwned + Validate + Send + Sync,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(value) =
            axum::extract::Query::<T>::from_request_parts(parts, state)
                .await
                .map_err(|e| e.into_response())?;

        value
            .validate()
            .map_err(|e| crate::error::ApiError::Validation(e).into_response())?;

        Ok(Query(value))
    }
}

impl<T: OpenApiType> OpenApiExtractor for Query<T> {
    fn modify_operation(op: &mut Operation) {
        let schema = T::schema();
        for (name, prop_schema) in schema.properties {
            let required = schema.required.contains(&name);
            op.parameters.push(crate::openapi::Parameter {
                name,
                in_: "query".to_string(),
                required,
                schema: prop_schema,
            });
        }
    }
}

pub trait HeaderName: Send + Sync {
    fn name() -> &'static str;
}

pub struct Header<T: HeaderName, V = String>(pub V, std::marker::PhantomData<T>);

impl<T: HeaderName, V> std::ops::Deref for Header<T, V> {
    type Target = V;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<T, V, S> FromRequestParts<S> for Header<T, V>
where
    T: HeaderName,
    V: std::str::FromStr + Send + Sync,
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let name = T::name();
        let value = parts.headers.get(name).ok_or_else(|| {
            crate::error::ApiError::BadRequest(format!("Missing header: {}", name)).into_response()
        })?;

        let value_str = value.to_str().map_err(|_| {
            crate::error::ApiError::BadRequest(format!("Invalid header encoding: {}", name))
                .into_response()
        })?;

        let parsed = value_str.parse::<V>().map_err(|_| {
            crate::error::ApiError::BadRequest(format!("Invalid header format: {}", name))
                .into_response()
        })?;

        Ok(Header(parsed, std::marker::PhantomData))
    }
}

impl<T: HeaderName, V: OpenApiType> OpenApiExtractor for Header<T, V> {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: T::name().to_string(),
            in_: "header".to_string(),
            required: true,
            schema: V::schema(),
        });
    }
}
