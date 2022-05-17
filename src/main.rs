#[macro_use]
extern crate mybatis;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate async_trait;

mod config;
mod domain;
mod dao;
mod service;
mod api;
mod common;

use actix_web::{get, web, App, HttpServer, Responder};
use api::user_api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/hello", web::get().to(|| async { "api server run successed" }))
            .service(user_api::register)
    })
    .bind(("127.0.0.1", 8082))?
    .run()
    .await
}