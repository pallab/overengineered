mod config;
mod routes;
mod models;
mod db;
mod schema;
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
    println!("Starting the app ..........");
    println!("Starting the app ..........");

    let config_file = std::env::var("CONFIG_FILE").unwrap_or(String::from("config.json"));

    let config = load_config(config_file);

    println!("config is {}", serde_json::to_string(&config).unwrap());

    let url = std::env::var("DATABASE_URL").unwrap_or("".to_owned());
    println!("db url is {} ", url);
    // let app_state = web::Data::new(AppState{
    //     counter : Mutex::new(0)
    // });

    let mut db_conn = db::create_connection(&url);

    let users = db::table::users::list_users(&mut db_conn);

    let usrs : Vec<String> = users.iter().map(|u| serde_json::json!(u).to_string()).collect();

    println!("Users are : \n{}", usrs.join("\n") );

    HttpServer::new(move || {
        App::new()
            //.app_data(app_state.clone())
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(Files::new("/", "./static"))
    })
        .bind((config.host, config.port))?
        .run()
        .await

   // std::io::Result::Ok(())
}