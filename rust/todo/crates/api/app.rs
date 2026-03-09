use axum::{
    routing::{get, post},
    Router
};
use crate::state::AppState;
use crate::todo::{create_todo, list_todos};

pub fn build_app(state: AppState) -> Router {
    let app = axum::Router::new().route(
        "/health",
        axum::routing::get(|| async { "ok" }),
    ).route("/todos", post(create_todo))
    .route("/todos", get(list_todos))
    .with_state(state);
    app
}