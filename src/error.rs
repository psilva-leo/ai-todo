use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use std::collections::HashMap;
use thiserror::Error;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub message: String,
    pub status: u16,
    pub errors: HashMap<String, Vec<String>>,
}

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Resource not found")]
    NotFound,

    // Add this variant
    #[error("Validation failed")]
    InvalidInput(HashMap<String, Vec<String>>),

    #[error("Something went wrong: {0}")]
    Internal(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message, errors) = match self {
            AppError::InvalidInput(errs) => (
                StatusCode::BAD_REQUEST,
                "Validation failed".to_string(),
                errs,
            ),
            AppError::NotFound => (
                StatusCode::NOT_FOUND,
                "Resource not found".to_string(),
                HashMap::new(),
            ),
            AppError::Internal(s) => (StatusCode::INTERNAL_SERVER_ERROR, s, HashMap::new()),
        };

        let body = Json(ErrorResponse {
            message,
            status: status.as_u16(),
            errors,
        });

        (status, body).into_response()
    }
}
