use error_stack::{Context, Report, Result, ResultExt};
use serde::{Deserialize, Serialize};
use std::{fmt, fs::File, io::Read};

pub use auth::AuthConfig;
pub use debug::DebugConfig;
pub use postgres::PostgresConfig;
pub use redis::RedisConfig;
pub use web_server::WebServerConfig;

mod auth;
mod debug;
mod postgres;
mod redis;
mod web_server;

const CONFIG_NAME: &'static str = "config.toml";
const CONFIG_VERSION: u8 = 1;

#[derive(Debug)]
pub struct ConfigError;

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Config error")
    }
}

impl Context for ConfigError {}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TriceratopsConfig {
    version: u8,
    #[serde(rename = "web-server")]
    web_server: WebServerConfig,
    debug: DebugConfig,
    postgres: PostgresConfig,
    redis: RedisConfig,
    auth: AuthConfig,
}

impl TriceratopsConfig {
    pub fn load() -> Result<Self, ConfigError> {
        let mut config_file = File::open(CONFIG_NAME)
            .map_err(Report::from)
            .attach_printable("Failed to open config file")
            .change_context(ConfigError)?;

        let mut config_string = String::new();

        config_file
            .read_to_string(&mut config_string)
            .map_err(Report::from)
            .attach_printable("Failed to parse config file to text")
            .change_context(ConfigError)?;

        let config = toml::from_str::<TriceratopsConfig>(&config_string.as_str())
            .map_err(Report::from)
            .attach_printable("Failed to parse config")
            .change_context(ConfigError)?;

        if config.version != CONFIG_VERSION {
            return Err(Report::new(ConfigError))
                .attach_printable("Config version mismatch")
                .attach_printable_lazy(|| {
                    format!(
                        "Expected version {}, found {}",
                        CONFIG_VERSION, config.version
                    )
                });
        }

        Ok(config)
    }

    pub fn web_server(&self) -> &WebServerConfig {
        &self.web_server
    }

    pub fn debug(&self) -> &DebugConfig {
        &self.debug
    }

    pub fn postgres(&self) -> &PostgresConfig {
        &self.postgres
    }

    pub fn redis(&self) -> &RedisConfig {
        &self.redis
    }

    pub fn auth(&self) -> &AuthConfig {
        &self.auth
    }
}
