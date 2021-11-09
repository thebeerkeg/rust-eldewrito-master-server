use actix_web::{Responder, get, web, HttpResponse};
use crate::models::database::Database;
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

// announcing servers to the server browser
#[get("/list")]
pub async fn list(data: web::Data<Database>) -> impl Responder {
    let result = data.handle_list();

    HttpResponse::Ok().json(Response {
        result
    })
}
