use std::{fmt::Display, time::Duration};

use deadpool_redis::{Manager, Pool as RedisPool, Runtime, Timeouts};
use error_stack::{Context, Report, Result, ResultExt};

#[derive(Debug)]
pub struct CacheError;

impl Display for CacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Cache module error")
    }
}

impl Context for CacheError {}

#[derive(Debug, Clone)]
pub struct Cache;

impl Cache {
    pub async fn new() -> Result<RedisPool, CacheError> {
        let redis_url = std::env::var("REDIS_URL")
            .map_err(Report::from)
            .attach_printable("Failed to read variable REDIS_URL")
            .change_context(CacheError)?;

        let manager = Manager::new(redis_url.to_owned())
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
