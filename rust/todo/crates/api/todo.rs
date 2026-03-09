use axum::{
    extract::State,
    Json,
};

use models::{CreateTodo, Todo};
use crate::state::AppState;


pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Json<Todo> {
    let todo = services::create(&state.db, payload.title)
        .await
        .unwrap();

    Json(todo)
}

pub async fn list_todos(
    State(state): State<AppState>,
) -> Json<Vec<Todo>> {
    let todos = services::list(&state.db)
        .await
        .unwrap();

    Json(todos)
}