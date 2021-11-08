use actix_web::{Responder, get, web, HttpResponse};
use crate::models::database::Database;
use std::collections::HashSet;
use serde::{Serialize};

// is set in the eldewrito client (30 + 2 * 60)
const ED_SERVER_CONTACT_TIME_LIMIT: u8 = 150;

#[derive(Serialize)]
pub struct Response {
    result: Result
}

#[derive(Serialize)]
pub struct Result {
    code: u8,
    servers: HashSet<String>,
    msg: String
}

// announcing servers to the server browser
#[get("/list")]
pub async fn list(data: web::Data<Database>) -> impl Responder {
    let servers: HashSet<String> = data.announces.lock().unwrap().iter()
        .filter_map(|announce| {
            // server should announce every 150 secs,
            // but let's give them a margin of another 150 secs
            match announce.timestamp.elapsed().unwrap().as_secs() < 2 * ED_SERVER_CONTACT_TIME_LIMIT as u64 {
                true => Some(announce.as_addr()),
                false => None
            }
        })
        .collect();

    HttpResponse::Ok().json(Response {
        result: Result {
            code: 0,
            servers,
            msg: "OK".to_string()
        }
    })
}
