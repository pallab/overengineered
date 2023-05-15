mod config;
mod routes;
mod models;
mod db;
mod schema;

use std::sync::{Arc};
use actix_files::Files;
use crate::config::load_config;
use actix_web::{App, web, HttpServer};
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager};
use diesel::MysqlConnection;
use env_logger::Env;

use tonic::{transport::Server, Request, Response, Status};
use messenger::{ListFilesRequest};
use messenger::messenger_client::MessengerClient;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;

pub mod messenger {
    tonic::include_proto!("messenger");
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the app. Environment variables are :");
//    std::env::vars().for_each(|v| println!("{} : {}", v.0, v.1));

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // load the config file
    let config_file = std::env::var("CONFIG_FILE").unwrap_or(String::from("config.json"));
    let config = load_config(config_file);

    println!("config is {}", serde_json::to_string(&config).unwrap());

    let mut grpc_client = MessengerClient::connect("http://[::1]:8089").await.expect("");

    let request = tonic::Request::new(
        ListFilesRequest { }
    );

    let response = grpc_client.list_files(request).await.expect("");
    println!("grpc response {:?}", response);







    let manager = ConnectionManager::<MysqlConnection>::new(config.db_url);

    let pool: Arc<DbPool> = Arc::new(r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("database URL should be valid"));

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(routes::list_users)
            .service(Files::new("/", "./static"))
    })
        .bind((config.host, config.port))?
        .run()
        .await;

   std::io::Result::Ok(())
}