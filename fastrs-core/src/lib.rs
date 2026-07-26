pub mod app;
pub mod error;
pub mod extractors;
pub mod openapi;
pub mod responses;

#[cfg(feature = "htmx")]
pub mod htmx;

pub use app::{App, Method, RouteDef};
pub use error::{ApiError, IntoApiError};
pub use extractors::{AuthVerifier, Bearer, Header, HeaderName, Json, Page, Path, Query, State};
pub use openapi::{
    MediaType, OpenApi, OpenApiExtractor, OpenApiResponder, OpenApiType, Operation, Parameter,
    RequestBody, Response, Schema,
};
pub use responses::{Created, NoContent};

#[cfg(feature = "htmx")]
pub use htmx::{HxRedirect, HxRefresh, HxRequest, HxTarget, HxTrigger};

#[doc(hidden)]
pub use axum;
