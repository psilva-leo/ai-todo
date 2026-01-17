use axum::{
    Router,
    routing::{delete, get, patch, post},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use crate::{routes, state::AppState};

pub fn create_app(state: AppState) -> Router {
    Router::new()
        .route("/todos", post(routes::todos::create_todo))
        .route("/todos", get(routes::todos::list_todos))
        .route("/todos/:id", get(routes::todos::get_todo))
        .route("/todos/:id", patch(routes::todos::update_todo))
        .route("/todos/:id", delete(routes::todos::delete_todo))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}
