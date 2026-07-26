# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- `App::layer<L>()` generic tower middleware escape hatch; `with_cors()` and `with_tracing()` refactored to use it internally (Part A)
- htmx integration via `axum-htmx` behind the `htmx` feature flag; `OpenApiExtractor` wrappers for `HxRequest`, `HxTarget`, `HxTrigger`, `HxRedirect`, `HxRefresh`; `examples/htmx-todo/` (Part B)
- `examples/todo-api-postgres/` — full CRUD Todo API backed by Postgres via sqlx; DB error → ApiError pattern documented (Part C)
- `with_rate_limit(RateLimitConfig)` — rate limiting via `rate_rs`, returns 429 on breach (Part D)
- `health_check(path)` and `health_check_with(path, check_fn)` — simple and custom health check endpoints, 503 on check failure (Part D)
- `with_request_id()` — UUID v4 `X-Request-Id` header, integrated with tracing spans (Part D)
- `Multipart` extractor for `multipart/form-data` (Part D)
- Graceful shutdown on SIGINT/SIGTERM in `.run()` (Part D)

## [0.1.0] - Previous Release

### Added (v1 baseline)
- **Typed Error Handling**: `ApiError` enum and `IntoApiError` trait to seamlessly map errors into appropriate HTTP responses.
- **Validation Error Shape**: 422 Unprocessable Entity responses from `Json` and `Query` extractors are now formatted precisely as `{"errors": [{"field": "name", "message": "error text"}]}`.
- **Extractors**: Added `Query<T>` (with validation) and `Header` (with typed parsing) extractors.
- **Shared State**: Implemented `.with_state()` on the `App` builder to allow extracting Axum `State` inside handlers, fully typed.
- **Route Nesting**: Added `.nest()` method to mount sub-apps under a specific path, automatically prepending paths to OpenAPI schema operations.
- **Testing and CI**: Added integration tests (`tests/integration.rs`) using `tower::ServiceExt` and set up standard CI (`.github/workflows/ci.yml`).
