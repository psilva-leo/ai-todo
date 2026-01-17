mod app;
mod error;
mod models;
mod routes;
mod state;

use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() {
    init();

    let state = state::AppState::new();
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
