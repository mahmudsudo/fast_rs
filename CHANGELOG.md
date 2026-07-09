# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- **Typed Error Handling**: `ApiError` enum and `IntoApiError` trait to seamlessly map errors into appropriate HTTP responses.
- **Validation Error Shape**: 422 Unprocessable Entity responses from `Json` and `Query` extractors are now formatted precisely as `{"errors": [{"field": "name", "message": "error text"}]}`.
- **Extractors**: Added `Query<T>` (with validation) and `Header` (with typed parsing) extractors.
- **Shared State**: Implemented `.with_state()` on the `App` builder to allow extracting Axum `State` inside handlers, fully typed.
- **Route Nesting**: Added `.nest()` method to mount sub-apps under a specific path, automatically prepending paths to OpenAPI schema operations.
- **Testing and CI**: Added integration tests (`tests/integration.rs`) using `tower::ServiceExt` and set up standard CI (`.github/workflows/ci.yml`).
