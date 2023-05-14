use std::sync::Arc;
use actix_web::{get, web, error, post, App, HttpResponse, HttpServer, Responder, Result, Error, ResponseError};
use actix_files::NamedFile;

use crate::{db, DbPool};
use crate::models::{NewUser, User};

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
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

// #[get("/count")]
// async fn counter(app_data : web::Data<AppState>) -> impl Responder {
//     let mut counter = app_data.counter.lock().unwrap();
//     *counter +=1;
//
//     let str = json!({ "count" : *counter}).to_string()  ;
//
//     HttpResponse::Ok()
//         .content_type(ContentType::json())
//         .body( str)
// }

// #[get("/users/{user_id}/{friend}")] // <- define path parameters
// async fn index(path: web::Path<(u32, String)>) -> Result<String> {
//     let (user_id, friend) = path.into_inner();
//     Ok(format!("Welcome {}, user_id {}!", friend, user_id))
// }

