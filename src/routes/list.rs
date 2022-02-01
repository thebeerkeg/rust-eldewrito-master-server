use actix_web::{Responder, web, HttpResponse};
use crate::database::Database;
use std::collections::HashSet;
use serde::{Serialize};

#[derive(Serialize)]
pub struct Response {
    result: Result
}

#[derive(Serialize)]
pub struct Result {
    pub code: u8,
    pub servers: HashSet<String>,
    pub msg: String
}

// getting servers for server browser
pub async fn list(data: web::Data<Database>) -> impl Responder {
    let result = data.handle_list();

    HttpResponse::Ok().json(Response {
        result
    })
}
