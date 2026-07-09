use crate::openapi::{OpenApi, Operation};
use axum::{routing::MethodRouter, Router};

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

pub struct RouteDef {
    pub path: &'static str,
    pub method: Method,
    pub router: MethodRouter,
    pub operation: Operation,
}

pub struct App {
    router: Router,
    openapi: OpenApi,
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

impl App {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            openapi: OpenApi::new(),
        }
    }

    pub fn route(mut self, route_def: fn() -> RouteDef) -> Self {
        let def = route_def();
        
        // Convert OpenAPI format /users/{id} to Axum format /users/:id
        let mut axum_path = def.path.to_string();
        while let Some(start) = axum_path.find('{') {
            if let Some(end) = axum_path[start..].find('}') {
                let end = start + end;
                let param_name = &axum_path[start+1..end].to_string();
                axum_path.replace_range(start..=end, &format!(":{}", param_name));
            } else {
                break;
            }
        }

        self.router = self.router.route(&axum_path, def.router);

        let path_item = self.openapi.paths.entry(def.path.to_string()).or_default();
        let method_str = match def.method {
            Method::Get => "get",
            Method::Post => "post",
            Method::Put => "put",
            Method::Delete => "delete",
        };
        path_item.insert(method_str.to_string(), def.operation);

        self
    }

    pub fn serve_docs_at(mut self, path: &'static str) -> Self {
        let openapi_json = serde_json::to_string(&self.openapi).unwrap();
        
        let json_path = format!("{}/openapi.json", path);
        let openapi_json_cloned = openapi_json.clone();
        
        self.router = self.router.route(&json_path, axum::routing::get(move || async move {
            axum::response::Json(serde_json::from_str::<serde_json::Value>(&openapi_json_cloned).unwrap())
        }));

        let html = format!(r#"
<!DOCTYPE html>
<html>
<head>
    <title>Swagger UI</title>
    <link rel="stylesheet" type="text/css" href="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui.css">
</head>
<body>
    <div id="swagger-ui"></div>
    <script src="https://unpkg.com/swagger-ui-dist@5.9.0/swagger-ui-bundle.js"></script>
    <script>
        window.onload = function() {{
            window.ui = SwaggerUIBundle({{
                url: "{}",
                dom_id: '#swagger-ui',
            }});
        }}
    </script>
</body>
</html>
        "#, json_path);
        
        self.router = self.router.route(path, axum::routing::get(move || async move {
            axum::response::Html(html)
        }));

        self
    }

    pub fn run(self, addr: &str) {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            axum::serve(listener, self.router).await.unwrap();
        });
    }

    pub fn into_router(self) -> Router {
        self.router
    }
}
