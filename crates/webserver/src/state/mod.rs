use self::geo_ip::GeoIp;
use database::Database;
use deadpool_redis::Pool as RedisPool;
use error_stack::{Context, Report, Result, ResultExt};
use oauth::OAuthProviders;
use redis::Cache;
use sea_orm::DatabaseConnection;
use std::{fmt::Display, sync::Arc};

mod database;
mod geo_ip;
mod oauth;
mod redis;

#[derive(Debug)]
pub struct StateError;

impl Display for StateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("State error")
    }
}

impl Context for StateError {}

pub struct InternalAppState {
    pool: DatabaseConnection,
    cache: RedisPool,
    oauth: OAuthProviders,
    mm_db: Option<GeoIp>,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Result<Self, StateError> {
        let pool = Database::new()
            .await
            .attach_printable("State cannot be built without database connection")
            .change_context(StateError)?;

        let cache = Cache::new()
            .await
            .attach_printable("State cannot be built cache cache connection")
            .change_context(StateError)?;

        let oauth = OAuthProviders::new()
            .map_err(Report::from)
            .attach_printable("Failed to build OAuth clients")
            .change_context(StateError)?;

        let mm_db = GeoIp::new()
            .await
            .attach_printable("Failed to build GeoIp module")
            .change_context(StateError)?;

        Ok(Self {
            pool,
            cache,
            oauth,
            mm_db,
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

    pub fn mm_db(&self) -> &Option<GeoIp> {
        &self.mm_db
    }
}
