use actix_web::{Responder, post, web, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::database::Database;

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
    pub team: i64,
    pub player_index: i64,
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
    pub score: i64,
    pub kills: i64,
    pub assists: i64,
    pub deaths: i64,
    pub betrayals: i64,
    pub time_spent_alive: i64,
    pub suicides: i64,
    pub best_streak: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayerMedal {
    pub medal_name: String,
    pub count: i64,
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

// announcing servers to the server browser
#[post("/submit")]
pub async fn submit(request: web::Json<SubmitRequest>, data: web::Data<Database>) -> impl Responder {
    let submit_request = request.into_inner();
    let _response = data.handle_submit(&submit_request).await;
    HttpResponse::Ok()
}
