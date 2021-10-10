use config::{Environment, ConfigError};
use serde::{Deserialize};

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

#[derive(Deserialize)]
pub struct PgConfig {
    pub url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub pg: PgConfig,
}

impl Config {
    pub fn from_env() -> Result<Config, ConfigError> {
        let mut cfg = config::Config::new();
        cfg.merge(Environment::new())?;
        cfg.try_into()
    }
}