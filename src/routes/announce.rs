use actix_web::{web, get, Responder, HttpRequest, HttpResponse};
use std::time::SystemTime;
use::serde::{Serialize};
use crate::models::announce::Announce;
use crate::models::database::Database;
use crate::models::server::Server;

#[derive(Serialize)]
pub struct Response {
    result: Result
}

#[derive(Serialize)]
pub struct Result {
    pub code: u8,
    pub msg: String
}

// announcing servers to the server browser
#[get("/announce")]
pub async fn announce(server: web::Query<Server>, req: HttpRequest, data: web::Data<Database>) -> impl Responder {
    let result = data.handle_announce(Announce {
        server: server.into_inner(),
        socket_addr: req.peer_addr(),
        timestamp: SystemTime::now()
    });

    HttpResponse::Ok().json(Response {
        result
    })
}
