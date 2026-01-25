mod app;
mod error;
mod models;
mod routes;
mod services;
mod state;
mod validator;

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing_subscriber::{EnvFilter, fmt};

#[tokio::main]
async fn main() {
    dotenv().ok();

    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));
    fmt().with_env_filter(filter).init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations if enabled (mostly for local dev)
    let run_migrations = env::var("RUN_MIGRATIONS").is_ok_and(|v| v == "true");

    if run_migrations {
        tracing::info!("Running migrations...");
        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        tracing::info!("Migrations executed successfully.");
    }

    let gemini =
        services::gemini::GeminiService::new().expect("Failed to initialize GeminiService");
    let state = state::AppState::new(pool, gemini);
    let app = app::create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:5000")
        .await
        .unwrap();

    tracing::info!("Listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        () = ctrl_c => {},
        () = terminate => {},
    }

    eprintln!("signal received, starting graceful shutdown");

    let timeout_secs = std::env::var("SHUTDOWN_TIMEOUT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

    tokio::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_secs(timeout_secs)).await;
        eprintln!("shutdown timed out, forcing exit");
        std::process::exit(1);
    });
}
