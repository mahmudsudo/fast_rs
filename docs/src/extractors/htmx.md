# HTMX Extractors

When using `fastrs` with HTMX, you can leverage dedicated extractors like `HxRequest`, `HxTarget`, and `HxTrigger` to examine incoming HTMX-specific headers.

```toml
[dependencies]
fastrs = { git = "https://github.com/mahmudsudo/fast_rs.git", features = ["htmx"] }
```

```rust
use fastrs::{get, HxRequest, HxTarget};

#[get("/demo")]
async fn htmx_demo(
    hx_req: Option<HxRequest>,
    hx_target: Option<HxTarget>,
) -> &'static str {
    if hx_req.is_some() {
        "Response for HTMX"
    } else {
        "Standard Response"
    }
}
```

### Caveats & Notes
* Wrapping the extractors in `Option` allows them to handle requests that are not sent by HTMX.
* These extractors automatically register headers in the OpenAPI configuration.
