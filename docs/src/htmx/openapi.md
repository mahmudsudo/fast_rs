# OpenAPI Schema Reflection for HTMX

The HTMX extractors and responders automatically document their presence by contributing to the generated OpenAPI spec definitions.

```toml
[dependencies]
fastrs = { git = "https://github.com/mahmudsudo/fast_rs.git", features = ["htmx"] }
```

```rust
use fastrs::{get, HxRequest};

#[get("/api/htmx")]
async fn htmx_endpoint(hx: HxRequest) -> &'static str {
    "documented htmx endpoint"
}
```

### Caveats & Notes
* Interactive OpenAPI viewers like Swagger UI will list `HX-Request` headers automatically.
* Responses that return `HxRedirect` document a 302 status code response mapping.
