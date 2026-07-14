use fastrs::{
    App, ApiError, Created, Json, NoContent, Page, Path, get, post, patch, delete,
};
use serde::{Deserialize, Serialize};
use validator::Validate;
use std::sync::Mutex;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, fastrs::OpenApi)]
struct TodoResponse {
    id: i64,
    title: String,
    done: bool,
}

#[derive(Debug, Serialize, fastrs::OpenApi)]
struct TodoListResponse {
    items: Vec<TodoResponse>,
    total: i64,
    page: u32,
    limit: u32,
}

#[derive(Debug, Deserialize, Validate, fastrs::OpenApi)]
struct CreateTodoRequest {
    #[validate(length(min = 1))]
    title: String,
    #[serde(default)]
    done: bool,
}

#[derive(Debug, Deserialize, Validate, fastrs::OpenApi)]
struct UpdateTodoRequest {
    title: Option<String>,
    done: Option<bool>,
}

lazy_static::lazy_static! {
    static ref TODOS: Mutex<HashMap<i64, TodoResponse>> = Mutex::new(HashMap::new());
    static ref NEXT_ID: Mutex<i64> = Mutex::new(1);
}

#[post("/api/v1/todos")]
async fn create_todo(body: Json<CreateTodoRequest>) -> Result<Created<Json<TodoResponse>>, ApiError> {
    let mut todos = TODOS.lock().unwrap();
    let mut next_id = NEXT_ID.lock().unwrap();

    let id = *next_id;
    *next_id += 1;

    let todo = TodoResponse {
        id,
        title: body.title.clone(),
        done: body.done,
    };

    todos.insert(id, todo.clone());
    Ok(Created(Json(todo)))
}

#[get("/api/v1/todos")]
async fn list_todos(page: Page) -> Result<Json<TodoListResponse>, ApiError> {
    let todos = TODOS.lock().unwrap();

    let offset = ((page.page - 1) * page.limit) as usize;
    let limit = page.limit as usize;

    let total = todos.len() as i64;
    let mut sorted: Vec<_> = todos.values().cloned().collect();
    sorted.sort_by(|a, b| b.id.cmp(&a.id));

    let items: Vec<TodoResponse> = sorted
        .iter()
        .skip(offset)
        .take(limit)
        .cloned()
        .collect();

    Ok(Json(TodoListResponse {
        items,
        total,
        page: page.page,
        limit: page.limit,
    }))
}

#[get("/api/v1/todos/{id}")]
async fn get_todo(Path(id): Path<i64>) -> Result<Json<TodoResponse>, ApiError> {
    let todos = TODOS.lock().unwrap();

    todos
        .get(&id)
        .cloned()
        .ok_or_else(|| ApiError::NotFound(format!("Todo {} not found", id)))
        .map(Json)
}

#[patch("/api/v1/todos/{id}")]
async fn update_todo(
    Path(id): Path<i64>,
    body: Json<UpdateTodoRequest>,
) -> Result<Json<TodoResponse>, ApiError> {
    let mut todos = TODOS.lock().unwrap();

    let existing = todos
        .get(&id)
        .cloned()
        .ok_or_else(|| ApiError::NotFound(format!("Todo {} not found", id)))?;

    let updated = TodoResponse {
        id,
        title: body.title.clone().unwrap_or(existing.title),
        done: body.done.unwrap_or(existing.done),
    };

    todos.insert(id, updated.clone());
    Ok(Json(updated))
}

#[delete("/api/v1/todos/{id}")]
async fn delete_todo(Path(id): Path<i64>) -> Result<NoContent, ApiError> {
    let mut todos = TODOS.lock().unwrap();

    if todos.remove(&id).is_none() {
        return Err(ApiError::NotFound(format!("Todo {} not found", id)));
    }

    Ok(NoContent)
}

#[tokio::main]
 async fn main() {
    let app = App::new()
        .route(create_todo)
        .route(list_todos)
        .route(get_todo)
        .route(update_todo)
        .route(delete_todo)
        .serve_docs_at("/docs");

    println!("Server running on http://0.0.0.0:8000");
    println!("OpenAPI docs at http://0.0.0.0:8000/docs");

    app.run("0.0.0.0:8000").await;
}
