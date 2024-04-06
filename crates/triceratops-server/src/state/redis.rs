use error_stack::{Context, Result, ResultExt};
use fred::{
    clients::RedisPool,
    interfaces::ClientLike,
    types::{Builder, RedisConfig as FredConfig},
};
use std::fmt;

use crate::config::RedisConfig;

#[derive(Debug)]
pub struct CacheError;

impl fmt::Display for CacheError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Cache module error")
    }
}

impl Context for CacheError {}

#[derive(Debug, Clone)]
pub struct Cache;

impl Cache {
    pub async fn new(config: &RedisConfig) -> Result<RedisPool, CacheError> {
        let redis_url = format!(
            "redis://:{}@{}:{}",
            config.password().as_ref().unwrap_or(&"".to_string()),
            config.ip(),
            config.port()
        );

        let fred_config = FredConfig::from_url(redis_url.as_str())
            .attach_printable("Failed to make redis config.")
            .change_context(CacheError)?;

        let client = Builder::from_config(fred_config)
            .build_pool(config.max_connections().clone())
            .attach_printable("Failed to build redis client.")
            .change_context(CacheError)?;

        client
            .init()
            .await
            .attach_printable("Failed to initialize redis client.")
            .change_context(CacheError)?;

        Ok(client)
    }
}
