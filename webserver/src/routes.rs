use std::sync::Arc;
use actix::Addr;
use actix_web::{get, web, error, post, App, HttpResponse, HttpServer, Responder, Result, Error, ResponseError};
use actix_files::NamedFile;
use actix_web::http::StatusCode;
use serde::Deserialize;

use tonic::{transport::Server, Request, Response, Status};

use crate::{db, DbPool};
use crate::config::RpcConfig;
use crate::models::{NewUser, User};
use crate::rpc_impl::rpc;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./ui/out/index.html").await.unwrap()
}

#[post("/login")]
pub async fn login(form: web::Json<NewUser>) -> impl Responder {
    println!("got login {}", serde_json::json!(form.0));
    HttpResponse::Ok().body("done")
}

#[get("users/list")]
pub async fn list_users(pool: web::Data<Arc<DbPool>>) -> impl Responder {
    let mut conn = &mut pool.get().unwrap();
    let usersm: Vec<User> = db::table::users::list_users(&mut conn)
        .map_err(|e| error::ErrorInternalServerError(e)).unwrap();

    HttpResponse::Ok().json(usersm)
}





