use async_trait::async_trait;

use crate::common::PlayerInfo;
use crate::ranking_server::submit::{Game, Player};

mod queries;
pub mod mysql;
pub mod sqlite;

#[async_trait]
pub trait Database: Sync + Send {
    async fn get_player_info(&self, player_uid: &str) -> Result<PlayerInfo, sqlx::Error>;
    async fn insert_team_scores(&self, game_id: i64, scores: &Vec<i64>) -> Result<(), ()>;
    async fn insert_game_and_get_id(&self, game_version: &str, server_name: &str, server_ip: &str, server_port: &str, host_player: &str, game: &Game) -> Result<i64, ()>;
    async fn insert_player(&self, uid: &str) -> Result<String, ()>;
    async fn insert_player_info(&self, player: &Player) -> Result<(), ()>;
    async fn insert_game_player_result(&self, game_id: i64, player: &Player, exp: u32) -> Result<(), ()>;
}
