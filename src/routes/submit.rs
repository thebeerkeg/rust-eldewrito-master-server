use actix_web::{Responder, web, HttpResponse, HttpRequest};
use serde::{Serialize, Deserialize};
use crate::rems::Rems;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubmitRequest {
    pub game_version: String,
    pub server_name: String,
    pub server_port: i64,
    pub port: i64,
    pub host_player: String,
    pub game: Game,
    pub players: Vec<Player>,
    pub quitters: Vec<Quitter>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Game {
    pub sprint_enabled: bool,
    pub sprint_unlimited_enabled: bool,
    pub max_players: i64,
    pub map_name: String,
    pub map_file: String,
    pub variant: String,
    pub variant_type: String,
    pub team_game: bool,
    pub team_scores: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub client_name: String,
    pub service_tag: String,
    pub ip: String,
    pub team: u8,
    pub player_index: u8,
    pub uid: String,
    pub primary_color: String,
    pub player_game_stats: PlayerGameStats,
    pub player_medals: Vec<PlayerMedal>,
    pub player_weapons: Vec<PlayerWeapon>,
    pub other_stats: OtherStats,
    pub player_versus_player_kills: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerGameStats {
    pub score: u16,
    pub kills: u16,
    pub assists: u16,
    pub deaths: u16,
    pub betrayals: u16,
    pub time_spent_alive: u16,
    pub suicides: u16,
    pub best_streak: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMedal {
    pub medal_name: String,
    pub count: u16,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerWeapon {
    pub weapon_name: String,
    pub weapon_index: i64,
    pub kills: i64,
    pub killed_by: i64,
    pub betrayals_with: i64,
    pub suicides_with: i64,
    pub headshots_with: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherStats {
    pub nemesis_index: i64,
    pub kings_killed: i64,
    pub humans_infected: i64,
    pub zombies_killed: i64,
    pub time_in_hill: i64,
    pub time_controlling_hill: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quitter {
    pub uid: String,
    pub name: String,
}

// submit game results
pub async fn submit(request: web::Json<SubmitRequest>, req: HttpRequest, rems: web::Data<Rems>) -> impl Responder {
    let submit_request = request.into_inner();
    match rems.handle_submit(&submit_request, req.peer_addr()).await {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::Unauthorized()
    }
}
