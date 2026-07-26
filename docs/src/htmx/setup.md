# Enabling the HTMX Feature

To use HTMX extractors and responders in `fastrs`, you must enable the `htmx` feature flag in your `Cargo.toml`.

```toml
[dependencies]
fastrs = { git = "https://github.com/mahmudsudo/fast_rs.git", features = ["htmx"] }
```

### Caveats & Notes
* Enabling this feature pulls in dependencies related to `axum-htmx`.
* Ensure that the feature flag is consistently set across development and deployment targets.
