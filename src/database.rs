use crate::models::announce::Announce;
use std::sync::Mutex;
use crate::routes;
use std::collections::{HashMap};
use std::time::SystemTime;
use crate::models::server_info::ServerInfo;
use crate::routes::stats::{PlayerEntry, RankEmblemList, StatsRequest};
use crate::utils::http_client;
use sqlx;
use sqlx::SqlitePool;
use sqlx::sqlite::SqlitePoolOptions;
use crate::response::PlayerInfo;
use crate::routes::submit::{Game, Player, SubmitRequest};
use crate::utils::rank::calc_rank;

// is set in the eldewrito client (30 + 2 * 60)
const ED_SERVER_CONTACT_TIME_LIMIT_SECS: u64 = 150;

// update server list on /list request when last updated > x secs
const SERVER_LIST_UPDATE_INTERVAL_SECS: u64 = 10;

// max eldewrito rank
const MAX_RANK: u8 = 42;

// default emblem for new players
const DEFAULT_EMBLEM: &str = "";

#[derive(Debug)]
pub struct Database {
    announces: Mutex<Vec<Announce>>,
    server_list: Mutex<HashMap<String, SystemTime>>,
    server_list_last_updated: Mutex<SystemTime>,
    pool: SqlitePool
}

impl Database {
    pub async fn new() -> Self {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite://data.db?mode=rwc")
            .await
            .expect("Unable to create database pool.");

        sqlx::migrate!().run(&pool).await.expect("Could not run database migrations.");

        Self {
            announces: Mutex::new(Vec::new()),
            server_list: Mutex::new(HashMap::new()),
            server_list_last_updated: Mutex::new(SystemTime::now()),
            pool
        }
    }

    pub async fn handle_announce(&self, announce: Announce) -> routes::announce::Result {
        if announce.server.shutdown == Some(true) {
            self.server_list.lock().unwrap().remove(&*announce.server_addr());
            return routes::announce::Result {
                code: 0,
                msg: "Removed server from list.".to_string()
            }
        }

        // request server info
        let server_info_result = http_client::get::<ServerInfo>(format!("http://{}/", announce.server_addr())).await;

        if server_info_result.is_err() {
            return routes::announce::Result {
                code: 2,
                msg: "Failed to retrieve server info.".to_string()
            }
        }

        // you can do stuff with the server info if you want
        //let server_info = server_info_result.unwrap();

        self.server_list.lock().unwrap().insert(announce.server_addr(), announce.timestamp);

        routes::announce::Result {
            code: 0,
            msg: "OK".to_string()
        }
    }

    pub fn handle_list(&self) -> routes::list::Result {
        if self.server_list_last_updated.lock().unwrap().elapsed().unwrap().as_secs() > SERVER_LIST_UPDATE_INTERVAL_SECS {
            self.update_server_list();
        }

        routes::list::Result {
            code: 0,
            servers: self.server_list.lock().unwrap().keys().cloned().collect(),
            msg: "OK".to_string()
        }
    }

    fn update_server_list(&self) {
        // server should announce every 150 secs.
        // this removes all servers that haven't been re-announced in 300 secs.
        self.server_list.lock().unwrap().retain(|_, v| v.elapsed().unwrap().as_secs() < 2 * ED_SERVER_CONTACT_TIME_LIMIT_SECS);
        *self.server_list_last_updated.lock().unwrap() = SystemTime::now();
    }

    pub async fn handle_stats(&self, stats_request: &StatsRequest) -> RankEmblemList {
        let mut re_list = RankEmblemList::new();

        for (index, player) in stats_request.players.iter().enumerate() {
            let res: Result<PlayerInfo, sqlx::Error> = sqlx::query_as!(
                PlayerInfo,
                r#"SELECT name, service_tag, primary_color, IFNULL((SELECT SUM(exp) FROM game_player_results WHERE uid = $1), 0) as experience
                   FROM player_infos pi WHERE uid = $1
                "#,
                player.uid
            )
                .fetch_one(&self.pool)
                .await;

            if let Ok(player_info) = res {
                // don't want to have an integer overflow
                let experience = if let Some(exp) = player_info.experience {
                    exp.clamp(0, u32::MAX as i64) as u32
                } else {
                    0
                };

                // todo: get emblems from halostats
                re_list.insert(index.to_string(), PlayerEntry {
                    r: calc_rank(experience, MAX_RANK),
                    e: DEFAULT_EMBLEM.to_string()
                });
            } else {
                re_list.insert(index.to_string(), PlayerEntry {
                    r: 0,
                    e: DEFAULT_EMBLEM.to_string()
                });
            }
        }

        re_list
    }

    pub async fn insert_game_and_get_id(&self, game_version: &str, server_name: &str, host_player: &str, game: &Game) -> Result<i64, ()> {
        // todo: insert_team_scores(team_scores: Vec<i64>, game_id: i64)

        let res = sqlx::query!(
            r#"INSERT INTO games (game_version, server_name, host_player, map_name, map_file, variant, variant_type, team_game)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id as "game_id: i64""#,
            game_version,
            server_name,
            host_player,
            game.map_name,
            game.map_file,
            game.variant,
            game.variant_type,
            game.team_game
        )
            .fetch_one(&self.pool)
            .await.map_err(|_| ())?;

        Ok(res.game_id)
    }

    pub async fn insert_player(&self, uid: &str) -> Result<String, ()> {
        let _res = sqlx::query!(
            r#"INSERT OR IGNORE INTO players (uid)
            VALUES ($1)"#,
            uid
        )
            .execute(&self.pool)
            .await.map_err(|_| ())?;

        // can't fail
        Ok(uid.to_string())
    }

    pub async fn insert_player_info(&self, player: &Player) -> Result<(), ()> {
        let uid = self.insert_player(&player.uid).await?;

        let _res = sqlx::query!(
            r#"INSERT INTO player_infos (uid, ip, client_name, name, service_tag, primary_color)
            VALUES ($1, $2, $3, $4, $5, $6)"#,
            uid,
            player.ip,
            player.client_name,
            player.name,
            player.service_tag,
            player.primary_color
        )
            .execute(&self.pool)
            .await.map_err(|_| ())?;

        Ok(())
    }

    pub async fn insert_game_player_result(&self, game_id: i64, player: &Player, exp: u32) -> Result<(), ()> {
        let _res = sqlx::query!(
            r#"INSERT INTO game_player_results (game_id, uid, team, player_index, score, kills, assists, deaths, betrayals, time_spent_alive, suicides, best_streak, exp)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13)"#,
            game_id,
            player.uid,
            player.team,
            player.player_index,
            player.player_game_stats.score,
            player.player_game_stats.kills,
            player.player_game_stats.assists,
            player.player_game_stats.deaths,
            player.player_game_stats.betrayals,
            player.player_game_stats.time_spent_alive,
            player.player_game_stats.suicides,
            player.player_game_stats.best_streak,
            exp
        )
            .execute(&self.pool)
            .await.map_err(|_| ())?;

        Ok(())
    }

    pub async fn handle_submit(&self, submit_request: &SubmitRequest) -> Result<(), ()> {
        let game_id = self.insert_game_and_get_id(
            &submit_request.game_version,
            &submit_request.server_name,
            &submit_request.host_player,
            &submit_request.game
        ).await?;

        for player in submit_request.players.iter() {
            // todo: calc experience from game result logic
            let exp: u32 = 10;

            if self.insert_player_info(player).await.is_ok() {
                let _ = self.insert_game_player_result(game_id, player, exp).await;
            }
        }

        Ok(())
    }
}
