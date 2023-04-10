use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Todo {
    pub id: i32,
    pub date: i64,
    pub title: String,
    pub completed: bool,
}
