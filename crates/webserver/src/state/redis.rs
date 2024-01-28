use deadpool_redis::{Manager, Pool as RedisPool, Runtime};
use error_stack::{Context, Report, Result, ResultExt};
use std::{fmt, time::Duration};

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
        let redis_url = match config.password() {
            Some(pass) => format!("redis://:{}@{}:{}", pass, config.ip(), config.port()),
            None => format!("redis://{}:{}", config.ip(), config.port()),
        };

        let manager = Manager::new(redis_url)
            .map_err(Report::from)
            .attach_printable("Failed to build Redis pool manager")
            .change_context(CacheError)?;

        let pool_builder = RedisPool::builder(manager)
            .create_timeout(Some(Duration::from_secs(3)))
            .wait_timeout(Some(Duration::from_secs(3)))
            .recycle_timeout(Some(Duration::from_secs(120)))
            .max_size(100)
            .runtime(Runtime::Tokio1);

        let pool = pool_builder
            .build()
            .map_err(Report::from)
            .attach_printable("Failed to build Redis pool")
            .change_context(CacheError)?;

        Ok(pool)
    }
}
