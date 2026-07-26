# HTMX Extractors and Responders

`fastrs` includes first-class support for HTMX request verification and response-header manipulation via extractors like `HxRequest` and responders like `HxRedirect`.

```toml
[dependencies]
fastrs = { git = "https://github.com/mahmudsudo/fast_rs.git", features = ["htmx"] }
```

```rust
use fastrs::{get, HxRequest, HxRedirect};

#[get("/click")]
async fn click_handler(hx: Option<HxRequest>) -> HxRedirect {
    if hx.is_some() {
        HxRedirect("/clicked-success".to_string())
    } else {
        HxRedirect("/fallback".to_string())
    }
}
```

### Caveats & Notes
* If `HxRedirect` is used, the response returns an HTTP 200 with the `HX-Redirect` header instead of a standard redirect.
* `HxRefresh` is another responder that triggers a full page reload when evaluated.
