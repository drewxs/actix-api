use crate::todos::model::Todo;
use std::sync::Mutex;

pub struct AppState {
    pub todos: Mutex<Vec<Todo>>,
}
