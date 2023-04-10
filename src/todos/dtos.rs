use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub date: i64,
}

#[derive(Deserialize)]
pub struct UpdateTodo {
    pub title: String,
}
