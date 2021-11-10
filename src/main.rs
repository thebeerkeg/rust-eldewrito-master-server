mod models;
mod routes;
mod utils;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use crate::models::database::Database;

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("This is a master (chief) server for ElDewrito.")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db = web::Data::new(Database::new());

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(index)
            .service(routes::announce::announce)
            .service(routes::list::list)
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
