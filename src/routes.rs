use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use actix_web::http::header::ContentType;
use actix_files::NamedFile;
use config::FileFormat::Json;

use crate::models;

pub async fn index() -> impl Responder {
    NamedFile::open_async("./static/index.html").await.unwrap()
}

#[post("/login")]
pub async fn login(form: web::Json<models::NewUser>) -> impl Responder {
    println!("got login {}", serde_json::json!(form.0));

    HttpResponse::Ok().body("done")
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
