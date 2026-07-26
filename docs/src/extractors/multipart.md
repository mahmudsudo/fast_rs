# Multipart Extractor

The `Multipart` extractor handles `multipart/form-data` requests, making file uploads and field-by-field stream processing easy. It integrates with OpenAPI by specifying a binary input format.

```rust
use fastrs::{post, Multipart};

#[post("/upload")]
async fn upload_file(mut multipart: Multipart) -> &'static str {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let bytes = field.bytes().await.unwrap();
        println!("Received field {} with {} bytes", name, bytes.len());
    }
    "Upload complete"
}
```

### Caveats & Notes
* If parsing fails or the request is not multipart/form-data, an HTTP `400 Bad Request` is returned.
* The OpenAPI spec generates a schema indicating a binary payload representing the file structure.
