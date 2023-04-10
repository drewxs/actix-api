use actix_web::web;

use super::services::{create_todo, delete_todo, get_todos, update_todo};

pub fn route(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/todos")
            .service(get_todos)
            .service(create_todo)
            .service(update_todo)
            .service(delete_todo),
    );
}
