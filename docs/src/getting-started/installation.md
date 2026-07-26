# Installation

To start using `fastrs`, add the dependencies to your project's `Cargo.toml`. You will also need standard libraries like `tokio`, `serde`, and `validator` to leverage the full validation and async power.

```toml
[dependencies]
fastrs = { git = "https://github.com/mahmudsudo/fast_rs.git" }
tokio = { version = "1", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
validator = { version = "0.18", features = ["derive"] }
```

### Caveats & Notes
* Rust edition 2024 or higher is highly recommended.
* The `htmx` feature can be optionally enabled to unlock dedicated HTMX responders and extractors.
