use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Todo not found")]
    NotFound,

    #[error("Invalid input: {0}")]
    BadRequest(String),

    #[error("Something went wrong: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let status = match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        let body = json!({
            "error": self.to_string(),
            "status": status.as_u16()
        });

        (status, Json(body)).into_response()
    }
}
