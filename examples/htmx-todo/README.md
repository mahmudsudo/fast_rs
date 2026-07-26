# HTMX Todo Example

A minimal Todo page that returns a **partial HTML fragment** when the request includes the `HX-Request` header (htmx), and a **full HTML page** otherwise.

## Enable HTMX support

HTMX integration is **opt-in**. Add the `htmx` feature to your dependency:

```toml
[dependencies]
fastrs = { version = "0.1", features = ["htmx"] }
```

## Run

```bash
cargo run --example htmx-todo --features htmx
```

Then open http://0.0.0.0:8002/todos — click **Refresh** to see htmx swap the list without a full page load.

## Extractors

With the `htmx` feature, fastrs re-exports `HxRequest`, `HxTarget`, `HxTrigger`, `HxRedirect`, and `HxRefresh` from `axum-htmx`. Use them directly in handler signatures; OpenAPI header parameters are documented automatically.
