use std::str::FromStr;
use std::time::Duration;

use sqlx::mysql::{MySqlConnectOptions, MySqlPoolOptions};
use sqlx::{ConnectOptions, MySqlPool};
use async_trait::async_trait;

use crate::ranking_server::submit::{Game, Player};
use crate::common::PlayerInfo;
use crate::database::{Database, queries};

pub struct MySQLDatabase {
    pub pool: MySqlPool
}

impl MySQLDatabase {
    pub async fn new(db_url: &str) -> Self {
        let mut connection_options = MySqlConnectOptions::from_str(db_url).expect("Unable to create connection options.");

        connection_options
            .log_statements(log::LevelFilter::Error)
            .log_slow_statements(log::LevelFilter::Warn, Duration::from_secs(1));

        let db = MySqlPoolOptions::new()
            .connect_with(connection_options)
            .await
            .expect("Unable to create database pool.");

        sqlx::migrate!("migrations/mysql")
            .run(&db)
            .await
            .expect("Could not run database migrations.");

        Self { pool: db }
    }
}

#[async_trait]
impl Database for MySQLDatabase {
    async fn get_player_info(&self, player_uid: &str) -> Result<PlayerInfo, sqlx::Error> {
        sqlx::query_as::<_, PlayerInfo>(queries::mysql::GET_PLAYER_INFO)
            .bind(player_uid)
            .fetch_one(&self.pool)
            .await
    }

    async fn insert_team_scores(&self, game_id: i64, scores: &Vec<i64>) -> Result<(), ()> {
        let _res = sqlx::query(queries::mysql::INSERT_TEAM_SCORES)
            .bind(game_id)
            .bind(scores[0])
            .bind(scores[1])
            .bind(scores[2])
            .bind(scores[3])
            .bind(scores[4])
            .bind(scores[5])
            .bind(scores[6])
            .bind(scores[7])
            .execute(&self.pool)
            .await.map_err(|_| ())?;

        Ok(())
    }

    async fn insert_game_and_get_id(&self, game_version: &str, server_name: &str, server_ip: &str, server_port: &str, host_player: &str, game: &Game) -> Result<i64, ()> {
        let res: (i64,) = sqlx::query_as(queries::mysql::INSERT_GAME_AND_GET_ID)
            .bind(game_version)
            .bind(server_name)
            .bind(server_ip)
            .bind(server_port)
            .bind(host_player)
            .bind(&game.map_name)
            .bind(&game.map_file)
            .bind(&game.variant)
            .bind(&game.variant_type)
            .bind(&game.team_game)
            .fetch_one(&self.pool)
            .await
            .map_err(|_| ())?;

        Ok(res.0)
    }

    async fn insert_player(&self, uid: &str) -> Result<String, ()> {
        let _res = sqlx::query(queries::mysql::INSERT_PLAYER)
            .bind(uid)
            .execute(&self.pool)
            .await.map_err(|_| ())?;

        // can't fail
        Ok(uid.to_string())
    }

    async fn insert_player_info(&self, player: &Player) -> Result<(), ()> {
        sqlx::query(queries::mysql::INSERT_PLAYER_INFO)
            .bind(&player.uid)
            .bind(&player.ip)
            .bind(&player.client_name)
            .bind(&player.name)
            .bind(&player.service_tag)
            .bind(&player.primary_color)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|_| ())
    }

    async fn insert_game_player_result(&self, game_id: i64, player: &Player, exp: u32) -> Result<(), ()> {
        sqlx::query(queries::mysql::INSERT_GAME_PLAYER_RESULT)
            .bind(game_id)
            .bind(&player.uid)
            .bind(&player.team)
            .bind(&player.player_index)
            .bind(&player.player_game_stats.score)
            .bind(&player.player_game_stats.kills)
            .bind(&player.player_game_stats.assists)
            .bind(&player.player_game_stats.deaths)
            .bind(&player.player_game_stats.betrayals)
            .bind(&player.player_game_stats.time_spent_alive)
            .bind(&player.player_game_stats.suicides)
            .bind(&player.player_game_stats.best_streak)
            .bind(&player.uid)
            .bind(exp)
            .execute(&self.pool)
            .await
            .map(|_| ())
            .map_err(|_| ())
    }
}
