use axum::{
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
};
use rate_rs::{InMemoryStore, RateLimitConfig as RateRsConfig, RateLimitDecision, RateLimiter};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use std::time::Duration;
use tower::{Layer, Service};

/// Configuration for rate limiting endpoints.
///
/// Use this struct to define the maximum number of requests allowed within a specific time duration.
///
/// # Example
///
/// ```rust,ignore
/// use std::time::Duration;
/// use fastrs::RateLimitConfig;
///
/// let config = RateLimitConfig::new(100, Duration::from_secs(60));
/// ```
#[derive(Debug, Clone, Copy)]
pub struct RateLimitConfig {
    pub max_requests: u32,
    pub window: Duration,
}

impl RateLimitConfig {
    /// Creates a new `RateLimitConfig` with the specified request count and duration window.
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
        }
    }
}

/// Tower Layer for rate limiting powered by `rate_rs`.
///
/// This layer applies an in-memory sliding-window rate limit to requests.
///
/// # Example
///
/// ```rust,ignore
/// use fastrs::{RateLimitConfig, RateLimitLayer};
/// use std::time::Duration;
///
/// let config = RateLimitConfig::new(10, Duration::from_secs(1));
/// let layer = RateLimitLayer::new(config);
/// ```
#[derive(Clone)]
pub struct RateLimitLayer {

    config: RateLimitConfig,
    limiter: Arc<RateLimiter<InMemoryStore>>,
}

impl RateLimitLayer {
    pub fn new(config: RateLimitConfig) -> Self {
        let store = InMemoryStore::new();
        let rate_rs_config = RateRsConfig {
            capacity: config.max_requests,
            refill_tokens: config.max_requests,
            refill_interval: config.window,
        };
        let limiter = Arc::new(RateLimiter::new(store, rate_rs_config));
        Self { config, limiter }
    }

    pub fn config(&self) -> &RateLimitConfig {
        &self.config
    }
}

impl<S> Layer<S> for RateLimitLayer {
    type Service = RateLimitService<S>;

    fn layer(&self, inner: S) -> Self::Service {
        RateLimitService {
            inner,
            limiter: self.limiter.clone(),
        }
    }
}

#[derive(Clone)]
pub struct RateLimitService<S> {
    inner: S,
    limiter: Arc<RateLimiter<InMemoryStore>>,
}

impl<S, ReqBody> Service<Request<ReqBody>> for RateLimitService<S>
where
    S: Service<Request<ReqBody>, Response = Response> + Clone + Send + 'static,
    S::Future: Send + 'static,
    ReqBody: Send + 'static,
{
    type Response = Response;
    type Error = S::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, req: Request<ReqBody>) -> Self::Future {
        let limiter = self.limiter.clone();
        let fut = self.inner.call(req);
        Box::pin(async move {
            let is_allowed = matches!(
                limiter.check("global").await,
                Ok(RateLimitDecision::Allowed { .. })
            );

            if !is_allowed {
                Ok((StatusCode::TOO_MANY_REQUESTS, ()).into_response())
            } else {
                fut.await
            }
        })
    }
}
