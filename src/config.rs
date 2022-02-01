use std::fs;
use std::net::SocketAddr;
use config::{ConfigError, Config, File};
use std::path::Path;
use std::str::FromStr;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct MasterServer {
    pub ed_announce_interval: u16,
    pub update_interval: u16
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RankingServer {
    pub enabled: bool,
    pub max_rank: u8,
    pub default_emblem: String
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
                ed_announce_interval: 150,
                update_interval: 10
            },
            ranking_server: RankingServer {
                enabled: false,
                max_rank: 37,
                default_emblem: "".to_string()
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
