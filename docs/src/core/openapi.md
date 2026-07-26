# The OpenApi Derive

The `#[derive(OpenApi)]` macro analyzes the struct fields and their types to generate compile-time OpenAPI schemas. It works in tandem with the validation attributes to document bounds and requirements in the generated JSON.

```rust
use fastrs::OpenApi;

#[derive(OpenApi)]
struct UserQuery {
    username: String,
    page: Option<u32>,
}
```

### Caveats & Notes
* All fields must implement `OpenApiType` for compilation to succeed.
* Supported attributes include basic validation details like `email` and `length(min = X)`.
