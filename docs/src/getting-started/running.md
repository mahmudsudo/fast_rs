# Running the Server

Start your `fastrs` server using standard Cargo commands. Once started, you can access the automatically generated interactive Swagger API documentation directly in your browser.

```bash
# Start the server locally
cargo run

# Access interactive documentation at:
# http://127.0.0.1:3000/docs
```

### Caveats & Notes
* Port collisions will raise binding errors from `tokio::net::TcpListener`.
* Ensure that the host address matches the binding target in your production configuration.
