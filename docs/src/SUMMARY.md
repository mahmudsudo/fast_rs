# Summary

- [Introduction](introduction.md)

- [Getting Started]()
  - [Installation](getting-started/installation.md)
  - [Quickstart](getting-started/quickstart.md)
  - [Running the server](getting-started/running.md)

- [Core Concepts]()
  - [How routing macros work](core/routing.md)
  - [The OpenApi derive](core/openapi.md)
  - [Validation](core/validation.md)

- [Extractors]()
  - [Json<T>](extractors/json.md)
  - [Path<T>](extractors/path.md)
  - [Query<T>](extractors/query.md)
  - [Bearer<T> and AuthVerifier](extractors/auth.md)
  - [Multipart](extractors/multipart.md)
  - [htmx extractors](extractors/htmx.md)

- [State and Database]()
  - [with_state() pattern](database/state.md)
  - [DB integration pattern](database/pattern.md)

- [Error Handling]()
  - [ApiError](errors/api-error.md)
  - [Custom error types](errors/custom.md)

- [Middleware]()
  - [App::layer() escape hatch](middleware/layer.md)
  - [CORS](middleware/cors.md)
  - [Tracing](middleware/tracing.md)
  - [Rate limiting](middleware/rate-limit.md)
  - [Request ID](middleware/request-id.md)

- [Response Types]()
  - [Json<T>](responses/json.md)
  - [Created<T> and NoContent](responses/status.md)
  - [Pagination with Page<T>](responses/pagination.md)

- [htmx Integration]()
  - [Enabling the feature](htmx/setup.md)
  - [Extractors and responders](htmx/extractors.md)
  - [OpenAPI schema reflection](htmx/openapi.md)

- [API Versioning](versioning.md)
- [Health Checks](health.md)

- [Deployment]()
  - [Graceful shutdown](deployment/shutdown.md)
  - [Dockerfile](deployment/docker.md)
  - [Environment config](deployment/config.md)

- [Comparison](comparison.md)
