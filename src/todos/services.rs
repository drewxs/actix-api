use actix_web::{get, post, put, delete, web, Responder, HttpResponse};
use crate::{AppState, Todo};
use super::models::{CreateTodo, UpdateTodo};

#[get("/todos")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todos.lock().unwrap().to_vec())
}

#[post("/todos")]
async fn create_todo(data: web::Data<AppState>, param: web::Json<CreateTodo>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let mut max_id: i32 = 0;
    
    for i in 0..todos.len() {
        if todos[i].id > max_id {
            max_id = todos[i].id
        }
    }

    todos.push(Todo {
        id: max_id + 1,
        title: param.title.clone(),
        date: param.date,
    });

    HttpResponse::Ok().json(todos.to_vec())
}

#[put("/todos/{id}")]
async fn update_todo(data: web::Data<AppState>, path: web::Path<i32>, param: web::Json<UpdateTodo>) -> impl Responder {
    let id = path.into_inner();
    let mut todos = data.todos.lock().unwrap();

    for i in 0..todos.len() {
        if todos[i].id == id {
            todos[i].title = param.title.clone();
            break;
        }
    }

    HttpResponse::Ok().json(todos.to_vec())
}

#[delete("/todos/{id}")]
async fn delete_todo(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();

    *todos = todos.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todos.to_vec())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_todos)
        .service(create_todo)
        .service(update_todo)
        .service(delete_todo);
}
