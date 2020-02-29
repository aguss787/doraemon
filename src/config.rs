use std::fs;

use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Config {
    pub database: DatabaseConfig,
}

#[derive(Deserialize, Clone)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: i32,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub fn get_config() -> Config {
    let config = fs::read("var/config.toml").expect("Error in reading config file");
    toml::from_slice(&config).expect("Error in parsing config")
}
