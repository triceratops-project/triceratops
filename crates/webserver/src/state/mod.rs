use crate::config::TriceratopsConfig;
use database::Database;
use deadpool_redis::Pool as RedisPool;
use error_stack::{Context, Report, Result, ResultExt};
use oauth::OAuthProviders;
use redis::Cache;
use sea_orm::DatabaseConnection;
use std::{fmt, sync::Arc};

mod database;
mod oauth;
mod redis;

#[derive(Debug)]
pub struct StateError;

impl fmt::Display for StateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("State error")
    }
}

impl Context for StateError {}

pub struct InternalAppState {
    pool: DatabaseConnection,
    cache: RedisPool,
    oauth: OAuthProviders,
    config: TriceratopsConfig,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new(config: TriceratopsConfig) -> Result<Self, StateError> {
        let pool = Database::new(config.postgres())
            .await
            .attach_printable("State cannot be built without database connection")
            .change_context(StateError)?;

        let cache = Cache::new(config.redis())
            .await
            .attach_printable("State cannot be built cache cache connection")
            .change_context(StateError)?;

        let oauth = OAuthProviders::new(config.auth().oauth(), config.web_server().url())
            .await
            .map_err(Report::from)
            .attach_printable("Failed to build OAuth clients")
            .change_context(StateError)?;

        Ok(Self {
            pool,
            cache,
            oauth,
            config,
        })
    }

    pub fn pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    pub fn cache(&self) -> &RedisPool {
        &self.cache
    }

    pub fn oauth(&self) -> &OAuthProviders {
        &self.oauth
    }
}
