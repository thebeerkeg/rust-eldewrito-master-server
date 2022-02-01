use config::{ConfigError, Config, File};
use std::path::Path;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct MasterServer {
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RankingServer {
    pub enabled: bool,
    pub max_rank: u8,
    pub default_emblem: String
}

#[derive(Debug, Clone, Deserialize)]
pub struct RemsConfig {
    pub master_server: MasterServer,
    pub ranking_server: RankingServer
}

impl RemsConfig {
    pub fn new() -> Result<Self, ConfigError> {
        let mut config = Config::new();

        const CONFIG_PATH: &str = "./config.toml";

        if Path::new(CONFIG_PATH).exists() {
            config.merge(File::with_name(CONFIG_PATH))?;
        }

        match config.try_into() {
            Ok(data) => Ok(data),
            Err(e) => Err(ConfigError::Message(format!("Errors while processing config: {}.", e))),
        }
    }
}
