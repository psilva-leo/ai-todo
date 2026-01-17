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
};

#[derive(serde::Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub description: Option<String>,
    pub priority: Option<Priority>,
}

pub async fn create_todo(
    State(state): State<AppState>,
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    if payload.title.trim().is_empty() {
        return Err(AppError::BadRequest("title cannot be empty".into()));
    }

    let now = Utc::now();
    let todo = Todo {
        id: Uuid::new_v4(),
        title: payload.title,
        description: payload.description,
        status: TodoStatus::Todo,
        priority: payload.priority.unwrap_or(Priority::Medium),
        source: TodoSource::Manual,
        created_at: now,
        updated_at: now,
    };

    state.todos.lock().unwrap().insert(todo.id, todo.clone());

    Ok(Json(todo))
}

pub async fn list_todos(State(state): State<AppState>) -> Json<Vec<Todo>> {
    let todos = state.todos.lock().unwrap().values().cloned().collect();

    Json(todos)
}

pub async fn get_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<Json<Todo>, AppError> {
    let todo = state
        .todos
        .lock()
        .unwrap()
        .get(&id)
        .cloned()
        .ok_or(AppError::NotFound)?;

    Ok(Json(todo))
}

#[derive(serde::Deserialize)]
pub struct UpdateTodo {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<TodoStatus>,
    pub priority: Option<Priority>,
}

pub async fn update_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    let mut todos = state.todos.lock().unwrap();
    let todo = todos.get_mut(&id).ok_or(AppError::NotFound)?;

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

    Ok(Json(todo.clone()))
}

pub async fn delete_todo(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
) -> Result<(), AppError> {
    let removed = state.todos.lock().unwrap().remove(&id);
    if removed.is_none() {
        return Err(AppError::NotFound);
    }

    Ok(())
}
