use database::Database;
use deadpool_redis::Pool as RedisPool;
use oauth::OAuthProviders;
use redis::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod database;
mod oauth;
mod redis;

#[derive(Clone)]
pub struct InternalAppState {
    pool: DatabaseConnection,
    cache: RedisPool,
    oauth: OAuthProviders,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Self {
        let pool = Database::new().await;

        let cache = Cache::new().await;

        let oauth = OAuthProviders::default();

        Self { pool, cache, oauth }
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
