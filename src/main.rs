mod models;
mod routes;
mod utils;
pub mod database;
pub mod config;
pub mod response;

use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use database::Database;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a master (chief) server for ElDewrito.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::new().await);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(routes::announce::announce)
            .service(routes::list::list)
            .service(routes::submit::submit)
            .service(routes::stats::stats)
    })
        .bind("0.0.0.0:3000")?
        .run()
        .await
}
