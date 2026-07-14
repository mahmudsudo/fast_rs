use crate::openapi::{OpenApi, Operation};
use axum::{Router, routing::MethodRouter};
use tower_http::{
    cors::CorsLayer,
    trace::{TraceLayer, DefaultOnFailure, OnFailure},
    classify::MakeClassifier,
};

pub enum Method {
    Get,
    Post,
    Put,
    Patch,
    Delete,
}

pub struct RouteDef<S = ()> {
    pub path: &'static str,
    pub method: Method,
    pub router: MethodRouter<S>,
    pub operation: Operation,
}

pub struct App<S = ()> {
    router: Router<S>,
    pub openapi: OpenApi,
}

impl Default for App<()> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: Clone + Send + Sync + 'static> App<S> {
    pub fn new() -> Self {
        Self {
            router: Router::new(),
            openapi: OpenApi::new(),
        }
    }
    pub fn route(mut self, route_def: fn() -> RouteDef<S>) -> Self {
        let def = route_def();

        let mut axum_path = def.path.to_string();
        while let Some(start) = axum_path.find('{') {
            if let Some(end) = axum_path[start..].find('}') {
                let end = start + end;
                let param_name = &axum_path[start + 1..end].to_string();
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
            Method::Patch => "patch",
            Method::Delete => "delete",
        };
        path_item.insert(method_str.to_string(), def.operation);

        self
    }

    pub fn nest(mut self, path: &str, app: App<S>) -> Self {
        self.router = self.router.nest(path, app.router);

        for (sub_path, operations) in app.openapi.paths {
            let full_path = format!("{}{}", path, if sub_path == "/" { "" } else { &sub_path });
            let path_item = self.openapi.paths.entry(full_path).or_default();
            for (method, op) in operations {
                path_item.insert(method, op);
            }
        }

        self
    }

   pub fn with_state(self, state: S) -> App<()> {
    App {
        router: self.router.with_state(state),
        openapi: self.openapi,
    }
}

    pub fn with_cors(mut self, layer: CorsLayer) -> Self {
        self.router = self.router.layer(layer);
        self
    }

    pub fn with_tracing<L: std::clone::Clone+std::marker::Send+tower_http::classify::MakeClassifier+ 'static>(mut self, layer: TraceLayer<L>) -> Self where <L as MakeClassifier>::ClassifyEos: Send, DefaultOnFailure: OnFailure<<L as MakeClassifier>::FailureClass>, <L as MakeClassifier>::Classifier: Clone + Send, <L as MakeClassifier>::Classifier: 'static, <L as MakeClassifier>::ClassifyEos: 'static {
        self.router = self.router.layer(layer);
        self
    }

    pub fn serve_docs_at(mut self, path: &'static str) -> Self {
        let openapi_json = serde_json::to_string(&self.openapi).unwrap();

        let json_path = format!("{}/openapi.json", path);
        let openapi_json_cloned = openapi_json.clone();

        self.router = self.router.route(
            &json_path,
            axum::routing::get(move || async move {
                axum::response::Json(
                    serde_json::from_str::<serde_json::Value>(&openapi_json_cloned).unwrap(),
                )
            }),
        );

        let html = format!(
            r#"
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
        "#,
            json_path
        );

        self.router = self.router.route(
            path,
            axum::routing::get(move || async move { axum::response::Html(html) }),
        );

        self
    }

    pub fn into_router(self) -> Router<S> {
        self.router
    }
}

impl App<()> {
    pub async fn run(self, addr: &str) {
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        axum::serve(listener, self.router).await.unwrap();
    }
}