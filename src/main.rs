mod models;
mod routes;
mod utils;
mod database;
mod config;
mod response;

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
    let announce_endpoint = format!("/{}", cfg.master_server.announce_endpoint);
    let list_endpoint = format!("/{}", cfg.master_server.list_endpoint);
    let submit_endpoint = format!("/{}", cfg.ranking_server.submit_endpoint);
    let stats_endpoint = format!("/{}", cfg.ranking_server.stats_endpoint);


    let db = web::Data::new(Database::new(cfg).await);

    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(web::resource(&announce_endpoint).route(web::get().to(routes::announce::announce)))
            .service(web::resource(&list_endpoint).route(web::get().to(routes::list::list)))
            .service(web::resource(&submit_endpoint).route(web::post().to(routes::submit::submit)))
            .service(web::resource(&stats_endpoint).route(web::post().to(routes::stats::stats)))
            .service(index)
    })
        .bind(bind_address)?
        .run()
        .await

    // HttpServer::new(move || {
    //     App::new()
    //         .app_data(db.clone())
    //         .service(index)
    //         .service(routes::announce::announce)
    //         .service(routes::list::list)
    //         .service(routes::submit::submit)
    //         .service(routes::stats::stats)
    // })
    //     .bind(bind_address)?
    //     .run()
    //     .await
}
