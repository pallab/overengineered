mod config;
mod routes;
mod models;
mod db;
mod schema;
mod stocks_rpc;
mod rpc_impl;

use std::sync::{Arc};
use actix_files::Files;
use crate::config::load_config;
use actix_web::{App, web, HttpServer};
use diesel::r2d2;
use diesel::r2d2::{ConnectionManager};
use diesel::MysqlConnection;
use env_logger::Env;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the app. Environment variables are :");

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // load the config file
    let config_file = std::env::var("CONFIG_FILE").unwrap_or(String::from("config.json"));
    let config = load_config(config_file);

    println!("config is {}", serde_json::to_string(&config).unwrap());

    let manager = ConnectionManager::<MysqlConnection>::new(config.mysql.db_url);

    let pool: Arc<DbPool> = Arc::new(r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("database URL should be valid"));

    let rpc_config = Arc::new(config.rpc);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(routes::list_users)
            .service(
                web::scope("")
                    .app_data(web::Data::new(rpc_config.clone()))
                    .service(routes::list_stocks)
                    .service(routes::stock_price_ticks),
            )
            .service(Files::new("/", "./static"))
    })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}