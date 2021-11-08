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
    code: u8,
    msg: String
}

// announcing servers to the server browser
#[get("/announce")]
pub async fn announce(server: web::Query<Server>, req: HttpRequest, data: web::Data<Database>) -> impl Responder {
    let announce_result = &data.add_announce(Announce {
        server: server.into_inner(),
        socket_addr: req.peer_addr(),
        timestamp: SystemTime::now()
    });

    let result = match announce_result {
        Ok(_) => Result {
            code: 0,
            msg: "OK".to_string()
        },
        Err(_) => Result {
            code: 2,
            msg: "Server unreachable".to_string()
        }
    };

    HttpResponse::Ok().json(Response {
        result
    })
}
