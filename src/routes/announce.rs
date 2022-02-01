use std::net::IpAddr;
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use std::time::SystemTime;
use::serde::{Serialize, Deserialize};
use crate::common::Announce;
use crate::rems::Rems;

#[derive(Deserialize, Debug)]
pub struct Server {
    // can be either ipv4 or ipv6
    pub ip: IpAddr,
    pub port: u16,
    // remove server from database
    pub shutdown: Option<bool>,
}

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
pub async fn announce(server: web::Query<Server>, req: HttpRequest, rems: web::Data<Rems>) -> impl Responder {
    let result = rems.handle_announce(Announce {
        server: server.into_inner(),
        socket_addr: req.peer_addr(),
        timestamp: SystemTime::now()
    }).await;

    HttpResponse::Ok().json(Response {
        result
    })
}
