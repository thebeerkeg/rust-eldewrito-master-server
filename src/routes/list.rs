use crate::models::database::Database;
use rocket::State;
use rocket::serde::{Serialize, json::Json};
use std::collections::HashSet;
use std::iter::FromIterator;

#[derive(Serialize)]
pub struct Response<'r> {
    result: Result<'r>
}

#[derive(Serialize)]
pub struct Result<'r> {
    code: u8,
    servers: HashSet<String>,
    msg: &'r str
}

// announcing servers to the server browser
#[get("/list")]
pub async fn list(db: &State<Database>) -> Json<Response<'_>> {
    let servers: HashSet<String> = db.announces.lock().await.iter()
        .map(|announce| announce.as_addr())
        .collect();

    Json(Response {
        result: Result {
            code: 0,
            servers,
            msg: "OK"
        }
    })
}
