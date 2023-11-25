#![allow(unused)]

mod application;
mod domain;
mod infrastructure;
mod presentation;

use actix_web::{dev::Service, get, web, App, HttpResponse, HttpServer, Responder};
use application::task::commands::create::CreateTaskCommandHandler;
use futures_util::{future::FutureExt, lock::Mutex};
use infrastructure::{controllers::task::TaskController, persistence::TaskRepositoryLocal};
use presentation::rest::{routers::task_router, state::AppState};
use std::io;

#[actix_web::main]
async fn main() -> io::Result<()> {
    let task_cont = TaskController {
        repo: TaskRepositoryLocal::new(),
    };
    let state = web::Data::new(AppState {
        task_controller: task_cont,
    });
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
            .service(web::scope("/api/v1").configure(task_router))
            .service(handle_hello)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;
    Ok(())
}

#[get("/")]
async fn handle_hello() -> impl Responder {
    println!("Handler{:>25}", "hello_handler");
    HttpResponse::Ok().body("<h1>Hello</h1>")
}
