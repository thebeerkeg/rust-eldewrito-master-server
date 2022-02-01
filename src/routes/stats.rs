use actix_web::{Responder, web, HttpResponse};
use std::collections::{HashMap};
use serde::{Serialize, Deserialize};
use crate::rems::Rems;

// key is number in string, eg: "0", "15"
pub type RankEmblemList = HashMap<String, PlayerEntry>;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatsRequest {
    pub players: Vec<StatsRequestPlayer>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StatsRequestPlayer {
    pub name: String,
    pub filtered_name: String,
    pub service_tag: String,
    pub player_index: i64,
    pub uid: String,
    pub ip: String,
}

#[derive(Serialize)]
pub struct PlayerEntry {
    pub r: u8,
    pub e: String,
}

// get rank and emblem of players
pub async fn stats(request: web::Json<StatsRequest>, data: web::Data<Rems>) -> impl Responder {
    let stats_request = request.into_inner();
    let re_list = data.handle_stats(&stats_request).await;
    HttpResponse::Ok().json(re_list)
}
