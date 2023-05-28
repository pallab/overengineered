mod config;
mod routes;
mod models;
mod db;
mod schema;
mod stocks_rpc;
mod rpc_impl;
mod actors;
mod kafka;
mod route_websocket;
mod canvas;
mod letters;

use std::sync::{Arc, Mutex};
use actix::Actor;
use actix_cors::Cors;
use actix_files::Files;
use crate::config::load_config;
use actix_web::{App, HttpServer, web};

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use env_logger::Env;
use log::*;
use crate::kafka::Kafka;

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

    info!("config is {}", serde_json::to_string(&config).unwrap());

    let manager = ConnectionManager::<MysqlConnection>::new(config.mysql.db_url);

    let pool: Arc<DbPool> = Arc::new(r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("database URL should be valid"));

    let rpc_config = Arc::new(config.rpc);
    let client = rpc_impl::rpc::new_client(&rpc_config.host, rpc_config.port).await.expect("");

    let kafka_client = Kafka::new();

    let addr = actors::leader::LeaderActor {
        rpc_client: Some(client),
        kafka_client : Some(kafka_client),
    }.start();

    // addr.do_send(actors::leader::Start);

    let sys_addr = Arc::new(addr);

    HttpServer::new(move || {
        //let cors = Cors::default().allow_any_origin();

        App::new()
           // .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(sys_addr.clone()))
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(routes::list_users)
            .route("/ws", web::get().to(route_websocket::ws_route))
            .service(
                web::scope("")
                    .app_data(web::Data::new(Arc::clone(&rpc_config)))
                    .service(routes::list_stocks)
                    .service(routes::stock_price_ticks),
            )
            // .service(
            //     web::scope("/ws")
            //         .route("/data", web::get().to(route_websocket::ws_route))
            // )

            .service(Files::new("/", "./static"))
    })
        .bind((config.server.host, config.server.port))?
        .run()
        .await
}
