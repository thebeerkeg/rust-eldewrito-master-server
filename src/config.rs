use std::fs;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use config::{ConfigError, Config, File};
use std::path::Path;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterServer {
    pub enabled: bool,
    pub announce_endpoint: String,
    pub list_endpoint: String,
    pub ed_announce_interval: u16,
    pub max_time_without_announce: u16,
    pub update_interval: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RankingServer {
    pub enabled: bool,
    pub submit_endpoint: String,
    pub stats_endpoint: String,
    pub submit_whitelist_enabled: bool,
    pub submit_whitelist: Vec<IpAddr>,
    pub default_emblem: String,
    pub max_rank: u8,
    pub winning_team_multiplier: u8,
    pub score_multiplier: u8,
    pub kills_multiplier: u8,
    pub assists_multiplier: u8,
    pub max_exp_per_game: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum DbDriver {
    SQLite,
    MySQL,
    Unknown
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemsConfig {
    pub log_level: Option<String>,
    pub bind_address: SocketAddr,
    pub external_address: Option<Ipv4Addr>,
    pub db_url: String,
    pub on_reverse_proxy: bool,
    pub master_server: MasterServer,
    pub ranking_server: RankingServer
}

impl RemsConfig {
    /// Default config
    pub fn default() -> Self {
        RemsConfig {
            log_level: Some("info".to_string()),
            bind_address: SocketAddr::from_str("0.0.0.0:3000").unwrap(),
            external_address: None,
            db_url: "sqlite://data.db?mode=rwc".to_string(),
            on_reverse_proxy: false,
            master_server: MasterServer {
                enabled: true,
                announce_endpoint: "announce".to_string(),
                list_endpoint: "list".to_string(),
                ed_announce_interval: 150,
                max_time_without_announce: 30,
                update_interval: 10
            },
            ranking_server: RankingServer {
                enabled: true,
                submit_endpoint: "submit".to_string(),
                stats_endpoint: "stats".to_string(),
                submit_whitelist_enabled: false,
                submit_whitelist: vec![IpAddr::from_str("127.0.0.1").unwrap()],
                default_emblem: "http://thebeerkeg.net/img/default.png".to_string(),
                max_rank: 37,
                winning_team_multiplier: 2,
                score_multiplier: 10,
                kills_multiplier: 1,
                assists_multiplier: 0,
                max_exp_per_game: 100
            }
        }
    }

    pub async fn load_from_file() -> Result<RemsConfig, ConfigError> {
        let mut config = Config::new();

        const CONFIG_PATH: &str = "config.toml";

        if Path::new(CONFIG_PATH).exists() {
            config.merge(File::with_name(CONFIG_PATH))?;
        } else {
            eprintln!("No config file found.");
            eprintln!("Creating config file..");
            let rems_config = RemsConfig::default();
            let _ = rems_config.save_to_file().await;
            return Err(ConfigError::Message(format!("Please edit the config.toml in the root folder and restart REMS.")))
        }

        config.try_into::<RemsConfig>()
    }

    pub async fn save_to_file(&self) -> Result<(), ()>{
        let toml_string = toml::to_string(self).expect("Could not encode TOML value");
        fs::write("config.toml", toml_string).expect("Could not write to file!");
        Ok(())
    }

    pub fn get_db_driver(&self) -> DbDriver {
        match &self.db_url.to_lowercase() {
            s if s.contains("mysql") => DbDriver::MySQL,
            s if s.contains("sqlite") => DbDriver::SQLite,
            _ => DbDriver::Unknown,
        }
    }
}
