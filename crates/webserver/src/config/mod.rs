use std::{fs::File, io::Write};

use auth::AuthConfig;
use postgres::PostgresConfig;
use redis::RedisConfig;
use serde::{Deserialize, Serialize};
use web_server::WebServerConfig;

mod auth;
mod postgres;
mod redis;
mod web_server;

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    web_server: WebServerConfig,
    postgres: PostgresConfig,
    redis: RedisConfig,
    auth: AuthConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            web_server: WebServerConfig::default(),
            postgres: PostgresConfig::default(),
            redis: RedisConfig::default(),
            auth: AuthConfig::default(),
        }
    }
}

impl Config {
    pub fn shit(&self) {
        let mut file = File::create("config.toml").unwrap();

        let config_string = toml::to_string(self).unwrap();

        file.write(config_string.as_bytes()).unwrap();
    }
}