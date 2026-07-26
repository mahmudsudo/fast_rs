use fastrs::axum::response::{Html, IntoResponse};
use fastrs::{App, HxRequest, get};

#[get("/todos")]
async fn todos_page(HxRequest(is_htmx): HxRequest) -> impl IntoResponse {
    let fragment = r##"<ul id="todo-list"><li>Buy milk</li><li>Walk the dog</li></ul>"##;

    if is_htmx {
        Html(fragment.to_string()).into_response()
    } else {
        let page = format!(
            r##"<!DOCTYPE html>
<html>
<head>
    <title>HTMX Todo</title>
    <script src="https://unpkg.com/htmx.org@2.0.4"></script>
</head>
<body>
    <h1>Todos</h1>
    <button hx-get="/todos" hx-target="#todo-list" hx-swap="outerHTML">Refresh</button>
    {fragment}
</body>
</html>"##
        );
        Html(page).into_response()
    }
}

#[get("/redirect-demo")]
async fn redirect_demo() -> fastrs::HxRedirect {
    fastrs::HxRedirect("/todos".to_string())
}

#[tokio::main]
async fn main() {
    let app = App::new().route(todos_page).route(redirect_demo);

    println!("HTMX Todo example running on http://0.0.0.0:8002");
    println!("Open http://0.0.0.0:8002/todos in a browser");

    app.run("0.0.0.0:8002").await;
}
