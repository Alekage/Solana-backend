#![allow(unused)]

use std::net::SocketAddr;
use axum::{extract::{Path, Query}, middleware, response::{Html, IntoResponse, Response}, routing::get, Router};
use model::ModelController;
use tower_cookies::CookieManagerLayer;
use serde::Deserialize;
use log::info;

mod error;
mod model;
mod web;

pub use crate::error::{LoginError, Result};

async fn main_response_mapper(res: Response) -> Response {
    res
}


#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let model_controler = ModelController::new().await?;

    // .merge() is used to merge the paths of two or more routers into a single Router

    // for a layer to be implemented over other routes and layers it has to be below them. 
    // layers are read bottom up!
    // layers are used to add additional processing to a requesto for a group of routes.
    let app = Router::new()
    .merge(web::routes_login::routes())
    // nest allows us to create routes which will all start with api/...
    // the model_controller clone is not deep clone sicne it has an Arc<Mutex<>>
    // It is just a +1 reference counter.
    .nest("/api", web::routes_ticket::routes(model_controler.clone()))
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new());
    

        
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    info!("Listening on address: {:?}", addr);
        // alternative:
        // let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
        // axum::serve(listener, app).await.unwrap();
    axum_server::bind(addr).serve(app.into_make_service()).await.unwrap();

    Ok(())
}