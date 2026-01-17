use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::models::todo::Todo;

#[derive(Clone)]
pub struct AppState {
    pub todos: Arc<Mutex<HashMap<Uuid, Todo>>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            todos: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}
