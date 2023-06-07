mod _config;
mod routes;
mod models;
mod db;
mod schema;
mod actors;
mod kafka;
mod route_websocket;
mod words_rpc_impl;
mod words_rpc;
mod config;

use std::sync::{Arc};
use actix_files::Files;
use actix_web::{App, HttpServer, web};

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use env_logger::Env;
use log::*;
use crate::actors::consumer::ConsumerActor;
use crate::actors::producer::ProducerActor;
use crate::config::{ KafkaConfig, MySqlConfig, ServerConfig};
use crate::kafka::KafkaAdmin;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the app. Environment variables are :");

    std::env::set_var("RUST_LOG", "info");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    let manager = ConnectionManager::<MysqlConnection>::new(
        MySqlConfig::db_url()
    );

    let pool: Arc<DbPool> = Arc::new(r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("database URL should be valid"));

    let res = KafkaAdmin::create_topic(
        vec![KafkaConfig::source_topic(), KafkaConfig::sink_topic()]).await;

    match res {
        Ok(r) => info!("created a new topic {:#?}",r),
        Err(e) => error!("failed to create the topic {}", e)
    }

    let _producer = ProducerActor::start();
    let consumer = Arc::new(ConsumerActor::start());

    HttpServer::new(move || {
        //let cors = Cors::default().allow_any_origin();

        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Arc::clone(&consumer)))
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(routes::list_users)
            .route("/ws", web::get().to(route_websocket::ws_route))
            .service(Files::new("/", "./ui/out"))
    })
        .bind((ServerConfig::host(), ServerConfig::port()))?
        .workers(4)
        .run()
        .await
}
