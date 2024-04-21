use actix_cors::Cors;
use actix_web::{App, get, HttpResponse, HttpServer, Responder, web};
use rust_eldewrito_master_server::config::RemsConfig;
use rust_eldewrito_master_server::{master_server, ranking_server};
use rust_eldewrito_master_server::rems::Rems;

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
    let master_server_enabled = cfg.master_server.enabled;
    let ranking_server_enabled = cfg.ranking_server.enabled;

    if !(master_server_enabled || ranking_server_enabled) { panic!("Master server and ranking server are disabled.") }

    let db = web::Data::new(Rems::new(cfg).await);

    println!("Started REMS on: {}", bind_address);

    HttpServer::new(move || {
        let mut app = App::new().app_data(db.clone());

        let cors = Cors::default().allow_any_origin().send_wildcard();

        app = app.wrap(cors);

        if master_server_enabled {
            app = app.service(web::resource(&announce_endpoint).route(web::get().to(master_server::announce::announce)))
                .service(web::resource(&list_endpoint).route(web::get().to(master_server::list::list)));
        }

        if ranking_server_enabled {
            app = app.service(web::resource(&submit_endpoint).route(web::post().to(ranking_server::submit::submit)))
                .service(web::resource(&stats_endpoint).route(web::post().to(ranking_server::stats::stats)));
        }

        app.service(index)
    })
        .bind(bind_address)?
        .run()
        .await
}
