use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Todo {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,

    #[sqlx(try_from = "String")]
    pub status: TodoStatus,

    #[sqlx(try_from = "String")]
    pub priority: Priority,

    #[sqlx(try_from = "String")]
    pub source: TodoSource,

    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TodoStatus {
    Todo,
    Doing,
    Done,
}

impl std::fmt::Display for TodoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoStatus::Todo => write!(f, "Todo"),
            TodoStatus::Doing => write!(f, "Doing"),
            TodoStatus::Done => write!(f, "Done"),
        }
    }
}

impl std::str::FromStr for TodoStatus {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Todo" => Ok(TodoStatus::Todo),
            "Doing" => Ok(TodoStatus::Doing),
            "Done" => Ok(TodoStatus::Done),
            _ => Err(format!("Invalid status: {s}")),
        }
    }
}

impl From<String> for TodoStatus {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(TodoStatus::Todo)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Priority {
    Low,
    Medium,
    High,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Low => write!(f, "Low"),
            Priority::Medium => write!(f, "Medium"),
            Priority::High => write!(f, "High"),
        }
    }
}

impl std::str::FromStr for Priority {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Low" => Ok(Priority::Low),
            "Medium" => Ok(Priority::Medium),
            "High" => Ok(Priority::High),
            _ => Err(format!("Invalid priority: {s}")),
        }
    }
}

impl From<String> for Priority {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(Priority::Medium)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TodoSource {
    Manual,
    Audio,
    Ai,
}

impl std::fmt::Display for TodoSource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TodoSource::Manual => write!(f, "Manual"),
            TodoSource::Audio => write!(f, "Audio"),
            TodoSource::Ai => write!(f, "Ai"),
        }
    }
}

impl std::str::FromStr for TodoSource {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Manual" => Ok(TodoSource::Manual),
            "Audio" => Ok(TodoSource::Audio),
            "Ai" => Ok(TodoSource::Ai),
            _ => Err(format!("Invalid source: {s}")),
        }
    }
}

impl From<String> for TodoSource {
    fn from(s: String) -> Self {
        s.parse().unwrap_or(TodoSource::Manual)
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedTodo {
    pub title: String,
    pub description: Option<String>,
    pub priority: Priority,
}
