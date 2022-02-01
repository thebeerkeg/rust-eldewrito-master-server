mod models;
mod routes;
mod utils;
pub mod database;
pub mod config;
pub mod response;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use database::Database;
use crate::config::RemsConfig;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a master (chief) server for ElDewrito.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let cfg = RemsConfig::load_from_file().await.expect("Could not load config.toml: ");
    let bind_address = cfg.bind_address.clone();

    let db = web::Data::new(Database::new(cfg).await);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(routes::announce::announce)
            .service(routes::list::list)
            .service(routes::submit::submit)
            .service(routes::stats::stats)
    })
        .bind(bind_address)?
        .run()
        .await
}
