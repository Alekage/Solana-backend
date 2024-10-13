use actix_web::{web, HttpResponse, Responder};


async fn get_app() -> impl Responder {
    HttpResponse::Ok().body("solana-app")
} 

pub fn app_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/app")
    .route(web::get().to(get_app))
    .route(web::head().to(HttpResponse::MethodNotAllowed)));
}

pub fn scoped_config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/test")
    .route(web::get().to(|| async { HttpResponse::Ok().body("solana-test") }))
    .route(web::head().to(HttpResponse::MethodNotAllowed)));
}