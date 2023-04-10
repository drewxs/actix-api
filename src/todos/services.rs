use super::{
    dtos::{CreateTodo, UpdateTodo},
    model::Todo,
};
use crate::AppState;
use actix_web::{delete, get, post, put, web, HttpResponse, Responder};

#[get("/")]
async fn get_todos(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().json(data.todos.lock().unwrap().to_vec())
}

#[post("/")]
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
        completed: false,
    });

    HttpResponse::Ok().json(todos.to_vec())
}

#[put("/{id}")]
async fn update_todo(
    data: web::Data<AppState>,
    path: web::Path<i32>,
    param: web::Json<UpdateTodo>,
) -> impl Responder {
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

#[delete("/{id}")]
async fn delete_todo(data: web::Data<AppState>, path: web::Path<i32>) -> impl Responder {
    let mut todos = data.todos.lock().unwrap();
    let id = path.into_inner();

    *todos = todos.to_vec().into_iter().filter(|x| x.id != id).collect();

    HttpResponse::Ok().json(todos.to_vec())
}
