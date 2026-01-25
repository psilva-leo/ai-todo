#[derive(Clone)]
pub struct AppState {
    pub pool: sqlx::PgPool,
    pub gemini: crate::services::gemini::GeminiService,
}

impl AppState {
    pub fn new(pool: sqlx::PgPool, gemini: crate::services::gemini::GeminiService) -> Self {
        Self { pool, gemini }
    }
}
