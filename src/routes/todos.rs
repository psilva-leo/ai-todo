use axum::{
    Json,
    extract::{Path, State},
};
use chrono::Utc;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::todo::{Priority, Todo, TodoSource, TodoStatus},
    state::AppState,
    validator::ValidatedJson,
};
use validator::Validate;

#[derive(serde::Deserialize, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "title cannot be empty"))]
    pub title: String,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub priority: Option<Priority>,
}

pub async fn create_todo(
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    let now = Utc::now();
    let new_todo = Todo {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: TodoStatus::Todo,
        priority: payload.priority.unwrap_or(Priority::Medium),
        source: TodoSource::Manual,
        created_at: now,
        updated_at: now,
    };

    let todo = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (id, title, description, status, priority, source, created_at, updated_at)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        RETURNING id, title, description, status, priority, source, created_at, updated_at
        "#,
        new_todo.id,
        new_todo.title,
        new_todo.description,
        new_todo.status.to_string(),
        new_todo.priority.to_string(),
        new_todo.source.to_string(),
        new_todo.created_at,
        new_todo.updated_at
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to create todo: {:?}", e);
        AppError::Internal("failed to create todo".into())
    })?;

    Ok(Json(todo))
}

pub async fn list_todos(State(state): State<AppState>) -> Result<Json<Vec<Todo>>, AppError> {
    let todos = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, description, status, priority, source, created_at, updated_at
        FROM todos
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to list todos: {:?}", e);
        AppError::Internal("failed to list todos".into())
    })?;

    Ok(Json(todos))
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Todo>, AppError> {
    let todo = sqlx::query_as!(
        Todo,
        r#"
        SELECT id, title, description, status, priority, source, created_at, updated_at
        FROM todos WHERE id = $1
        "#,
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to get todo: {:?}", e);
        AppError::Internal("failed to get todo".into())
    })?
    .ok_or(AppError::NotFound)?;

    Ok(Json(todo))
}

#[derive(serde::Deserialize, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1))]
    pub title: Option<String>,

    #[validate(length(max = 500))]
    pub description: Option<String>,

    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
}

pub async fn update_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    // Fetch first to apply partial updates
    let mut todo = sqlx::query_as!(
        Todo,
        "SELECT id, title, description, status, priority, source, created_at, updated_at FROM todos WHERE id = $1",
        id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to fetch todo for update: {:?}", e);
        AppError::Internal("failed to fetch todo".into())
    })?
    .ok_or(AppError::NotFound)?;

    if let Some(title) = payload.title {
        todo.title = title;
    }
    if let Some(desc) = payload.description {
        todo.description = Some(desc);
    }
    if let Some(status) = payload.status {
        todo.status = status;
    }
    if let Some(priority) = payload.priority {
        todo.priority = priority;
    }

    todo.updated_at = Utc::now();

    let updated_todo = sqlx::query_as!(
        Todo,
        r#"
        UPDATE todos 
        SET title = $1, description = $2, status = $3, priority = $4, updated_at = $5
        WHERE id = $6
        RETURNING id, title, description, status, priority, source, created_at, updated_at
        "#,
        todo.title,
        todo.description,
        todo.status.to_string(),
        todo.priority.to_string(),
        todo.updated_at,
        id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update todo: {:?}", e);
        AppError::Internal("failed to update todo".into())
    })?;

    Ok(Json(updated_todo))
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    let result = sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&state.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to delete todo: {:?}", e);
            AppError::Internal("failed to delete todo".into())
        })?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound);
    }

    Ok(())
}
