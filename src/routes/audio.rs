use axum::{
    Json,
    extract::{Multipart, State},
};
use serde::{Deserialize, Serialize};

use crate::{error::AppError, state::AppState};

#[derive(Debug, Deserialize)]
pub struct ConfirmTasksRequest {
    pub tasks: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SuggestedTasksResponse {
    pub tasks: Vec<String>,
}

pub async fn suggest_tasks(
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<Json<SuggestedTasksResponse>, AppError> {
    let mut audio_data = Vec::new();
    let mut mime_type = String::from("audio/mpeg"); // Default

    while let Some(field) = multipart
        .next_field()
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?
    {
        let name = field.name().unwrap_or_default().to_string();
        if name == "audio" {
            mime_type = field.content_type().unwrap_or("audio/mpeg").to_string();
            audio_data = field
                .bytes()
                .await
                .map_err(|e| AppError::Internal(e.to_string()))?
                .to_vec();
        }
    }

    if audio_data.is_empty() {
        return Err(AppError::Internal("No audio data provided".to_string()));
    }

    let tasks = state.gemini.suggest_tasks(audio_data, &mime_type).await?;

    Ok(Json(SuggestedTasksResponse { tasks }))
}

pub async fn confirm_tasks(
    State(state): State<AppState>,
    Json(req): Json<ConfirmTasksRequest>,
) -> Result<StatusCode, AppError> {
    for task_title in req.tasks {
        let id = uuid::Uuid::new_v4();
        let now = chrono::Utc::now();

        let status = crate::models::todo::TodoStatus::Todo.to_string();
        let priority = crate::models::todo::Priority::Medium.to_string();
        let source = crate::models::todo::TodoSource::Audio.to_string();

        sqlx::query!(
            "INSERT INTO todos (id, title, status, priority, source, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6, $7)",
            id,
            task_title,
            status,
            priority,
            source,
            now,
            now
        )
        .execute(&state.pool)
        .await
        .map_err(|e| AppError::Internal(e.to_string()))?;
    }

    Ok(StatusCode::CREATED)
}

use axum::http::StatusCode;
