use std::net::{SocketAddr};
use rocket::State;
use crate::models::server::Server;
use crate::models::database::Database;
use std::time::SystemTime;
use rocket::serde::{Serialize, json::Json};
use crate::models::announce::Announce;

#[derive(Serialize)]
pub struct Response<'r> {
    result: Result<'r>
}

#[derive(Serialize)]
pub struct Result<'r> {
    code: u8,
    msg: &'r str
}

// announcing servers to the server browser
#[get("/announce?<server..>")]
pub async fn announce(server: Server, db: &State<Database>, remote_addr: SocketAddr) -> Json<Response<'_>> {
    db.announces.lock().await.push(Announce {
        server,
        remote_ip: remote_addr,
        timestamp: SystemTime::now()
    });

    Json(Response {
        result: Result {
            code: 0,
            msg: "Added server to list"
        }
    })
}
