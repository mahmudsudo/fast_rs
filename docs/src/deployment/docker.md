# Dockerfile Configuration

For production deployment, use a multi-stage Dockerfile to build a small scratch or alpine image containing only the compiled binary.

```dockerfile
# Build stage
FROM rust:1.80-slim AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

# Final runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/fastrs-app /usr/local/bin/fastrs-app
EXPOSE 8000
CMD ["fastrs-app"]
```

### Caveats & Notes
* Use `recipe.json` with cargo-chef if you need to cache dependency compilation.
* Ensure you configure correct base libraries (like SSL certs) if your application communicates with external APIs.
