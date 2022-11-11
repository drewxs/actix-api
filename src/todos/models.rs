use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateTodo {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize, Clone)]
pub struct UpdateTodo {
    pub title: String,
}
