# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Auth Extractor**: `Bearer<T>` extractor for OAuth/JWT tokens with pluggable `AuthVerifier` trait. Automatically validates tokens and returns 401 on failure.
- **Pagination Extractor**: `Page` query extractor with sensible defaults (page=1, limit=20), bounds validation, and OpenAPI schema generation.
- **Response Status Codes**: `Created<T>` wrapper (201) and `NoContent` (204) response types with proper OpenAPI code generation.
- **Middleware Presets**: `.with_cors()` and `.with_tracing()` builder methods wrapping tower-http layers for common use cases.
- **HTTP Method**: Added support for PATCH requests via `#[patch]` macro.
- **Type Support**: Added `OpenApiType` implementations for `i64` and `serde_json::Value` for richer schema generation.
- **Full-featured Example**: New `examples/todo_api.rs` demonstrating all framework features: CRUD, pagination, validation, auth, and custom response codes.
- **Integration Tests**: Extended test suite covering new features (response codes, pagination bounds, bearer auth flow).

### Fixed
- State extractor now implements `OpenApiExtractor` with no-op to prevent breaking OpenAPI generation when used in handlers.

## [0.1.0] - Previous Release

### Added (v1 baseline)
- **Typed Error Handling**: `ApiError` enum and `IntoApiError` trait to seamlessly map errors into appropriate HTTP responses.
- **Validation Error Shape**: 422 Unprocessable Entity responses from `Json` and `Query` extractors are now formatted precisely as `{"errors": [{"field": "name", "message": "error text"}]}`.
- **Extractors**: Added `Query<T>` (with validation) and `Header` (with typed parsing) extractors.
- **Shared State**: Implemented `.with_state()` on the `App` builder to allow extracting Axum `State` inside handlers, fully typed.
- **Route Nesting**: Added `.nest()` method to mount sub-apps under a specific path, automatically prepending paths to OpenAPI schema operations.
- **Testing and CI**: Added integration tests (`tests/integration.rs`) using `tower::ServiceExt` and set up standard CI (`.github/workflows/ci.yml`).
