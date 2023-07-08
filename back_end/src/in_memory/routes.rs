use axum::{
    extract::{Path, State},
    routing::{get, patch},
    Json, Router,
};

use super::state::{AppState, NewTodo, Todo};

pub(crate) fn rest_router() -> Router<AppState> {
    Router::new()
        .route("/todos", get(get_todos).post(create_todo))
        .route("/todos/:id", patch(update_todo).delete(delete_todo))
}

async fn get_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    Json(state.todos.read().expect("mutex poisoned").clone())
}

async fn create_todo(
    State(state): State<AppState>,
    Json(todo): Json<NewTodo>,
) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");
    let todo = Todo {
        id: state.get_id(),
        title: todo.title,
        completed: false,
    };
    todos.push(todo);
    Json(todos.clone())
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(todo): Json<Todo>,
) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");

    if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos[index].completed = todo.completed;
        todos[index].title = todo.title.clone();
    }

    Json(todos.clone())
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> Json<Vec<Todo>> {
    let mut todos = state.todos.write().expect("mutex poisoned");
    if let Some(index) = todos.iter().position(|t| t.id == id) {
        todos.remove(index);
    }
    Json(todos.clone())
}
