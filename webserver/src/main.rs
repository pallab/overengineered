mod config;
mod routes;
mod models;
mod db;
mod schema;
mod actors;
mod kafka;
mod route_websocket;
mod words_rpc_impl;
mod words_rpc;

use std::sync::{Arc};
use actix_files::Files;
use crate::config::load_config;
use actix_web::{App, HttpServer, web};

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use env_logger::Env;
use log::*;
use crate::actors::consumer::ConsumerActor;
use crate::actors::producer::ProducerActor;
use crate::kafka::KafkaAdmin;

type DbPool = r2d2::Pool<ConnectionManager<MysqlConnection>>;
type PooledConn = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;
type DbError = Box<dyn std::error::Error + Send + Sync>;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting the app. Environment variables are :");

    std::env::set_var("RUST_LOG", "info");
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

    let rpc_config = Arc::new(config.rpc.clone());
    let kafka_config = Arc::new(config.kafka.clone());

    let admin_client = KafkaAdmin::new(config.kafka.clone());
    let res = admin_client.create_topic(
        vec![kafka_config.source_topic.clone(), kafka_config.sink_topic.clone()]).await;

    match res {
        Ok(r) => info!("created a new topic {:#?}",r),
        Err(e) => error!("failed to create the topic {}", e)
    }

    let _producer = ProducerActor::start(config.rpc, config.kafka.clone());
    let consumer = Arc::new(ConsumerActor::start(config.kafka));

    HttpServer::new(move || {
        //let cors = Cors::default().allow_any_origin();

        App::new()
           // .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(Arc::clone(&rpc_config)))
            .app_data(web::Data::new(Arc::clone(&kafka_config)))
            .app_data(web::Data::new(Arc::clone(&consumer)))
            .service(web::resource("/").to(routes::index))
            .service(routes::login)
            .service(routes::list_users)
            .route("/ws", web::get().to(route_websocket::ws_route))
            .service(Files::new("/", "./ui/out"))
    })
        .bind((config.server.host, config.server.port))?
        .workers(4)
        .run()
        .await
}
