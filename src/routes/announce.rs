use std::net::{IpAddr};
use actix_web::{web, Responder, HttpRequest, HttpResponse};
use::serde::{Serialize, Deserialize};
use crate::common::{IpWrapper};
use crate::rems::Rems;

#[derive(Deserialize, Debug)]
pub struct AnnounceRequest {
    // should be ignored for security reasons
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
pub async fn announce(request: web::Query<AnnounceRequest>, req: HttpRequest, rems: web::Data<Rems>) -> impl Responder {
    let announce_request = request.into_inner();
    let ip_wrapper = IpWrapper::from_req(&req);
    match rems.handle_announce(&announce_request, &ip_wrapper).await {
        Ok(_) => {
            HttpResponse::Ok().json(Response {
                result: Result { code: 0, msg: "Server added to list.".to_string() }
            })
        }
        Err(e) => {
            HttpResponse::Ok().json(Response {
                result: Result { code: 0, msg: e }
            })
        }
    }
}
