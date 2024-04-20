use std::net::{IpAddr};
use std::str::FromStr;
use actix_web::HttpRequest;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerInfoCompact {
    pub name: String,
    pub port: i64,
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

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct PlayerInfo {
    pub name: String,
    pub service_tag: String,
    pub primary_color: String,
    pub experience: Option<i64>
}

pub struct IpWrapper {
    pub forwarded_opt_ip: Option<IpAddr>,
    pub real_opt_ip: Option<IpAddr>
}

impl IpWrapper {
    pub fn from_req(req: &HttpRequest) -> Self {
        let forwarded_opt_ip_string = req.connection_info().realip_remote_addr().map(|s| s.to_string());

        let forwarded_opt_ip = if let Some(ip) = forwarded_opt_ip_string {
            IpAddr::from_str(&ip).ok()
        } else {
            None
        };

        let real_opt_ip = if let Some(socket) = req.peer_addr() {
            Some(socket.ip())
        } else {
            None
        };

        Self {
            forwarded_opt_ip,
            real_opt_ip
        }
    }
}
