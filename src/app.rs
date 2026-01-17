use axum::{
    Router,
    routing::{delete, get, patch, post},
};

use crate::{routes, state::AppState};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todos", post(routes::todos::create_todo))
        .route("/todos", get(routes::todos::list_todos))
        .route("/todos/:id", get(routes::todos::get_todo))
        .route("/todos/:id", patch(routes::todos::update_todo))
        .route("/todos/:id", delete(routes::todos::delete_todo))
        .with_state(state)
}
