pub use axum::extract::State;
use axum::{
    extract::{FromRequest, FromRequestParts, Request},
    response::{IntoResponse, Response},
};
use axum::http::header::AUTHORIZATION;
use serde::{Serialize, de::DeserializeOwned};
use std::collections::BTreeMap;
use validator::Validate;

use crate::error::ApiError;
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
#[axum::async_trait]
pub trait AuthVerifier<T>: Send + Sync + 'static {
    type Error: Into<ApiError>;

    async fn verify(&self, token: &str) -> Result<T, Self::Error>;
}

pub struct Bearer<T>(pub T);

impl<T> std::ops::Deref for Bearer<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[axum::async_trait]
impl<T, S> FromRequestParts<S> for Bearer<T>
where
    T: Send + Sync + 'static,
    S: AuthVerifier<T> + Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let auth_header = parts.headers.get(AUTHORIZATION).ok_or_else(|| {
            ApiError::Unauthorized("Missing Authorization header".to_string()).into_response()
        })?;

        let auth_str = auth_header.to_str().map_err(|_| {
            ApiError::BadRequest("Invalid Authorization header encoding".to_string()).into_response()
        })?;

        let token = auth_str.strip_prefix("Bearer ").ok_or_else(|| {
            ApiError::Unauthorized("Authorization header must use Bearer token".to_string()).into_response()
        })?;

        let value = state.verify(token).await.map_err(|err| err.into().into_response())?;
        Ok(Bearer(value))
    }
}

impl<T: OpenApiType> OpenApiExtractor for Bearer<T> {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: "Authorization".to_string(),
            in_: "header".to_string(),
            required: true,
            schema: crate::openapi::Schema {
                type_: Some("string".to_string()),
                ..Default::default()
            },
        });
    }
}

impl<T: OpenApiType> OpenApiResponder for Bearer<T> {
    fn modify_operation(_op: &mut Operation) {
        // Auth extractor does not affect response schema.
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

pub struct Page {
    pub page: u32,
    pub limit: u32,
}

impl std::ops::Deref for Page {
    type Target = Self;

    fn deref(&self) -> &Self::Target {
        self
    }
}

#[derive(serde::Deserialize)]
struct PageQuery {
    page: Option<u32>,
    limit: Option<u32>,
}

#[axum::async_trait]
impl<S> FromRequestParts<S> for Page
where
    S: Send + Sync,
{
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        let axum::extract::Query(value) =
            axum::extract::Query::<PageQuery>::from_request_parts(parts, state)
                .await
                .map_err(|e| e.into_response())?;

        let page = value.page.unwrap_or(1);
        let limit = value.limit.unwrap_or(20);

        if page == 0 {
            return Err(ApiError::BadRequest("page must be greater than 0".to_string()).into_response());
        }

        if limit == 0 {
            return Err(ApiError::BadRequest("limit must be greater than 0".to_string()).into_response());
        }

        Ok(Page { page, limit })
    }
}

impl OpenApiType for Page {
    fn schema() -> crate::openapi::Schema {
        let mut properties = BTreeMap::new();
        properties.insert(
            "page".to_string(),
            crate::openapi::Schema {
                type_: Some("integer".to_string()),
                ..Default::default()
            },
        );
        properties.insert(
            "limit".to_string(),
            crate::openapi::Schema {
                type_: Some("integer".to_string()),
                ..Default::default()
            },
        );

        crate::openapi::Schema {
            type_: Some("object".to_string()),
            properties,
            required: vec![],
            ..Default::default()
        }
    }
}

impl OpenApiExtractor for Page {
    fn modify_operation(op: &mut Operation) {
        op.parameters.push(crate::openapi::Parameter {
            name: "page".to_string(),
            in_: "query".to_string(),
            required: false,
            schema: crate::openapi::Schema {
                type_: Some("integer".to_string()),
                ..Default::default()
            },
        });
        op.parameters.push(crate::openapi::Parameter {
            name: "limit".to_string(),
            in_: "query".to_string(),
            required: false,
            schema: crate::openapi::Schema {
                type_: Some("integer".to_string()),
                ..Default::default()
            },
        });
    }
}

impl<T> OpenApiExtractor for State<T> {
    fn modify_operation(_op: &mut Operation) {
        // State is internal and doesn't appear in OpenAPI parameters.
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
