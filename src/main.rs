use actix_web::{get, web, App, HttpServer};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

mod todos;
use todos::services;

struct AppState {
    todos: Mutex<Vec<Todo>>
}

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: i32,
    date: i64,
    title: String,
}

#[get("/")]
async fn index() -> String {
    "Welcome to API".to_string()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_data = web::Data::new(AppState {
        todos: Mutex::new(vec![])
    });

    HttpServer::new(move || {
        App::new()
            .app_data(app_data.clone())
            .service(index)
            .configure(services::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run().await
}
