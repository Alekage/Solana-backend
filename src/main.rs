use std::sync::Mutex;
mod app_config;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use app_config::{app_config, scoped_config};

struct AppState {
    counter: Mutex<i32>
}

#[get("/users/{user_id}/{friend}")]
async fn get_user(path: web::Path<(u32, String)>) -> impl Responder {
    let (user_id, friend) = path.into_inner();
    HttpResponse::Ok().body(format!("Welcome {}, user_id {}!", friend, user_id))
 }

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

async fn index(data: web::Data<AppState>) -> String {
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Request number: {counter}")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let counter = web::Data::new(AppState { counter: Mutex::new(0) });

    HttpServer::new(move || {
        App::new().app_data(counter.clone())
        .configure(app_config)
        .service(web::scope("/api").configure(scoped_config))
        .service(get_user)
        .route("/", web::get().to(index))
    }).bind(("127.0.0.1", 3000))?.run().await
}
