mod config;
mod routes;
mod models;

use std::sync::Mutex;
use actix_files::Files;
use crate::config::load_config;
use serde_json::json;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};

struct AppState {
    counter: Mutex<i32>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting app");

    let config = load_config();

    println!("config is {}", config.name);

    // let app_state = web::Data::new(AppState{
    //     counter : Mutex::new(0)
    // });

    HttpServer::new(move || {
        App::new()
            //.app_data(app_state.clone())
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(Files::new("/", "./static"))
    })
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
    // std::io::Result::Ok(())
}