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
use validator::Validate;

use validator::ValidationErrors;

fn validation_error(err: ValidationErrors) -> AppError {
    let messages = err
        .field_errors()
        .iter()
        .map(|(field, errors)| {
            let msgs: Vec<_> = errors
                .iter()
                .filter_map(|e| e.message.as_ref())
                .map(|m| m.to_string())
                .collect();
            (field.to_string(), msgs)
        })
        .collect::<std::collections::HashMap<_, _>>();

    AppError::BadRequest(serde_json::to_string(&messages).unwrap())
}

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
    Json(payload): Json<CreateTodo>,
) -> Result<Json<Todo>, AppError> {
    payload.validate().map_err(validation_error)?;

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

    let mut todos = state
        .todos
        .lock()
        .map_err(|_| AppError::Internal("mutex poisoned".into()))?;
    todos.insert(todo.id, todo.clone());

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
    Json(payload): Json<UpdateTodo>,
) -> Result<Json<Todo>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::BadRequest(e.to_string()))?;

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
