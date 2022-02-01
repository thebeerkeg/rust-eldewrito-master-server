use std::fs;
use std::net::SocketAddr;
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
    pub default_emblem: String,
    pub max_rank: u8,
    pub winning_team_multiplier: u8,
    pub score_multiplier: u8,
    pub kills_multiplier: u8,
    pub assists_multiplier: u8
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RemsConfig {
    pub bind_address: SocketAddr,
    pub master_server: MasterServer,
    pub ranking_server: RankingServer
}

impl RemsConfig {
    /// Default config
    pub fn default() -> Self {
        RemsConfig {
            bind_address: SocketAddr::from_str("0.0.0.0:3000").unwrap(),
            master_server: MasterServer {
                enabled: false,
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
                default_emblem: "http://thebeerkeg.net/img/default.png".to_string(),
                max_rank: 37,
                winning_team_multiplier: 2,
                score_multiplier: 10,
                kills_multiplier: 1,
                assists_multiplier: 0
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
}
