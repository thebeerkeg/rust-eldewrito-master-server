use std::sync::Mutex;
use std::collections::HashMap;
use std::net::IpAddr;
use std::time::SystemTime;

use crate::master_server;
use crate::ranking_server::stats::{PlayerEntry, RankEmblemList, StatsRequest};
use crate::utils::http_client;
use crate::common::{ServerInfoCompact, IpWrapper};
use crate::config::{DbDriver, RemsConfig};
use crate::database::Database;
use crate::database::mysql::MySQLDatabase;
use crate::database::sqlite::SQLiteDatabase;
use crate::master_server::announce::AnnounceRequest;
use crate::ranking_server::submit::{Game, Player, SubmitRequest};

pub struct Rems {
    cfg: RemsConfig,
    server_list: Mutex<HashMap<String, SystemTime>>,
    server_list_last_updated: Mutex<SystemTime>,
    database: Box<dyn Database>
}

impl Rems {
    pub async fn new(cfg: RemsConfig) -> Self {
        let database: Box<dyn Database> = match cfg.get_db_driver() {
            DbDriver::SQLite => Box::new(SQLiteDatabase::new(&cfg.db_url).await),
            DbDriver::MySQL => Box::new(MySQLDatabase::new(&cfg.db_url).await),
            _ => panic!("Unknown database driver.")
        };

        Self {
            cfg,
            server_list: Mutex::new(HashMap::new()),
            server_list_last_updated: Mutex::new(SystemTime::now()),
            database
        }
    }

    pub fn calc_rank_from_exp(&self, experience: u32) -> u8 {
        (8.70 * (0.009 * (experience as f64) + 1.3).ln() - 2.35).floor().clamp(0.0, self.cfg.ranking_server.max_rank as f64) as u8
    }

    // will return forwarded ip if running behind reserve proxy
    // or external ip if announced ip is local
    pub fn get_real_ip(&self, ip_wrapper: &IpWrapper) -> Result<IpAddr, String> {
        if self.cfg.on_reverse_proxy && ip_wrapper.forwarded_opt_ip.is_none() {
            return Err("Reverse proxy did not forward ip.".to_string())
        } else if ip_wrapper.real_opt_ip.is_none() {
            return Err("Could not get ip of server.".to_string())
        }

        // can not fail because of prior verifications
        let mut ip = match self.cfg.on_reverse_proxy {
            true => ip_wrapper.forwarded_opt_ip.unwrap(),
            false => ip_wrapper.real_opt_ip.unwrap()
        };

        if !ip.is_global() {
            if let Some(external_ip) = self.cfg.external_address {
                ip = IpAddr::from(external_ip);
            } else {
                return Err("Can't list local ip.".to_string())
            }
        }

        Ok(ip)
    }

    pub async fn handle_announce(&self, announce_request: &AnnounceRequest, ip_wrapper: &IpWrapper) -> Result<String, String> {
        let ip = self.get_real_ip(ip_wrapper)?;

        let server_address = format!("{}:{}", ip, announce_request.port);

        if announce_request.shutdown == Some(true) {
            self.server_list.lock().unwrap().remove(&server_address);
            return Ok("Removed server from list.".to_string())
        }

        // request server info
        let server_info_result = http_client::get::<ServerInfoCompact>(format!("http://{}/", server_address)).await;

        if server_info_result.is_err() {
            return Err("Failed to retrieve server info.".to_string())
        }

        // you can do stuff with the server info if you want
        // let server_info = server_info_result.unwrap();

        self.server_list.lock().unwrap().insert(server_address, SystemTime::now());

        Ok("Server has been added.".to_string())
    }

    pub fn handle_list(&self) -> master_server::list::Result {
        if self.server_list_last_updated.lock().unwrap().elapsed().unwrap().as_secs() > self.cfg.master_server.update_interval as u64 {
            self.update_server_list();
        }

        master_server::list::Result {
            code: 0,
            servers: self.server_list.lock().unwrap().keys().cloned().collect(),
            msg: "OK".to_string()
        }
    }

    fn update_server_list(&self) {
        // this removes all servers that haven't been re-announced in time
        self.server_list.lock().unwrap().retain(|_, v| v.elapsed().unwrap().as_secs() < (self.cfg.master_server.ed_announce_interval + self.cfg.master_server.max_time_without_announce) as u64);
        *self.server_list_last_updated.lock().unwrap() = SystemTime::now();
    }

    pub async fn handle_stats(&self, stats_request: &StatsRequest) -> RankEmblemList {
        let mut re_list = RankEmblemList::new();

        for (index, player) in stats_request.players.iter().enumerate() {
            let res = self.database.get_player_info(&player.uid).await;

            if let Ok(player_info) = res {
                // don't want to have an integer overflow
                let experience = if let Some(exp) = player_info.experience {
                    exp.clamp(0, u32::MAX as i64) as u32
                } else {
                    0
                };

                re_list.insert(index.to_string(), PlayerEntry {
                    r: self.calc_rank_from_exp(experience),
                    e: self.cfg.ranking_server.default_emblem.to_string()
                });
            } else {
                re_list.insert(index.to_string(), PlayerEntry {
                    r: 0,
                    e: self.cfg.ranking_server.default_emblem.to_string()
                });
            }
        }

        re_list
    }

    pub async fn insert_team_scores(&self, game_id: i64, scores: &Vec<i64>) -> Result<(), ()> {
        self.database.insert_team_scores(game_id, scores).await
    }

    pub async fn insert_game_and_get_id(&self, game_version: &str, server_name: &str, server_ip: &str, server_port: &str, host_player: &str, game: &Game) -> Result<i64, ()> {
        let game_id = self.database.insert_game_and_get_id(game_version, server_name, server_ip, server_port, host_player, game).await?;

        if game.team_game {
            let _ = self.insert_team_scores(game_id, game.team_scores.as_ref().unwrap()).await;
        }

        Ok(game_id)
    }

    pub async fn insert_player(&self, uid: &str) -> Result<String, ()> {
        self.database.insert_player(uid).await
    }

    pub async fn insert_player_info(&self, player: &Player) -> Result<(), ()> {
        let _uid = self.insert_player(&player.uid).await?;

        self.database.insert_player_info(player).await
    }

    pub async fn insert_game_player_result(&self, game_id: i64, player: &Player, exp: u32) -> Result<(), ()> {
        self.database.insert_game_player_result(game_id, player, exp).await
    }

    pub fn calc_base_player_exp_from_game_result(&self, player: &Player) -> u32 {
        let score = player.player_game_stats.score.clamp(0, u16::MAX as i32) as u16; // objective points
        let kills = player.player_game_stats.kills.clamp(0, u16::MAX as i32) as u16;
        // let deaths = player.player_game_stats.deaths;
        let assists = player.player_game_stats.assists;
        // let best_streak = player.player_game_stats.best_streak; // broken stat

        // apply exp modifiers
        (
            // todo: add variant specific score to exp matcher
            // some variants give too much score
            (score * self.cfg.ranking_server.score_multiplier as u16).clamp(0, 50) +
            (kills * self.cfg.ranking_server.kills_multiplier as u16) +
            (assists * self.cfg.ranking_server.assists_multiplier as u16)
            // some variant types give too much score, so we set a max exp gain
        ).clamp(0, self.cfg.ranking_server.max_exp_per_game) as u32
    }

    pub async fn handle_submit(&self, submit_request: &SubmitRequest, ip_wrapper: &IpWrapper) -> Result<(), String> {
        let ip = self.get_real_ip(ip_wrapper)?;

        // whitelist check
        if self.cfg.ranking_server.submit_whitelist_enabled {
            if !self.cfg.ranking_server.submit_whitelist.contains(&ip) {
                return Err("Ip not whitelisted.".to_string())
            }
        }

        let game_id = self.insert_game_and_get_id(
            &submit_request.game_version,
            &submit_request.server_name,
            &ip.to_string(),
            &submit_request.server_port.to_string(),
            &submit_request.host_player,
            &submit_request.game
        ).await.map_err(|_| "Internal server error.".to_string())?;

        // todo: handle a draw as win, currently one of the drawing teams gets lucky
        let winning_team = if let Some(team_scores) = submit_request.game.team_scores.as_ref() {
            team_scores.iter().enumerate().max_by_key(|(_, &value)| value).map(|(idx, _)| idx).unwrap()
        } else {
            0
        };

        for player in submit_request.players.iter() {
            let base_exp = self.calc_base_player_exp_from_game_result(player);

            // apply winning team modifier
            let exp = if player.team as usize == winning_team {
                base_exp * self.cfg.ranking_server.winning_team_multiplier as u32
            } else {
                base_exp
            };

            if self.insert_player_info(player).await.is_ok() {
                let _ = self.insert_game_player_result(game_id, player, exp).await;
            }
        }

        Ok(())
    }
}
