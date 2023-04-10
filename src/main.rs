mod db;
mod todos;

use actix_web::{get, web, App, HttpResponse, HttpServer};
use db::AppState;
use std::sync::Mutex;

#[get("/")]
async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Welcome to API")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                todos: Mutex::new(vec![]),
            }))
            .service(index)
            .configure(todos::route)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
