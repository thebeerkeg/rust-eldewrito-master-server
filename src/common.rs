use std::net::SocketAddr;
use std::time::SystemTime;
use serde::{Serialize, Deserialize};
use crate::routes::announce::Server;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfo {
    pub name: String,
    pub port: i64,
    pub host_player: String,
    pub sprint_enabled: String,
    pub sprint_unlimited_enabled: String,
    pub dual_wielding: String,
    pub assassination_enabled: String,
    pub voting_enabled: bool,
    pub teams: bool,
    pub map: String,
    pub map_file: String,
    pub variant: String,
    pub variant_type: String,
    pub status: String,
    pub num_players: u8,
    pub mods: Vec<String>,
    pub max_players: u8,
    pub xnkid: String,
    pub xnaddr: String,
    pub players: Vec<Player>,
    pub is_dedicated: bool,
    pub game_version: String,
    pub eldewrito_version: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub name: String,
    pub service_tag: String,
    pub team: u8,
    pub uid: String,
    pub primary_color: String,
    pub is_alive: bool,
    pub score: i64,
    pub kills: i64,
    pub assists: i64,
    pub deaths: i64,
    pub betrayals: i64,
    pub time_spent_alive: i64,
    pub suicides: i64,
    pub best_streak: i64,
}

#[derive(Debug)]
pub struct Announce {
    pub server: Server,
    pub socket_addr: Option<SocketAddr>,
    pub timestamp: SystemTime,
}

impl Announce {
    pub fn server_addr(&self) -> String {
        let ip = match self.socket_addr {
            None => self.server.ip,
            Some(v) => v.ip()
        };

        format!("{}:{}", ip, self.server.port)
    }
}
