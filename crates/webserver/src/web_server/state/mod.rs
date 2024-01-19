use database::Database;
use deadpool_redis::Pool as RedisPool;
use redis::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use oauth::OAuthProviders;

mod database;
mod oauth;
mod redis;

pub struct InternalAppState {
    pool: DatabaseConnection,
    redis: RedisPool,
    oauth: OAuthProviders,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Self {
        let pool = Database::new().await;

        let redis = Cache::new().await;

        let oauth = OAuthProviders::new();

        Self { pool, redis, oauth }
    }

    pub fn get_pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    pub fn get_cache(&self) -> &RedisPool {
        &self.redis
    }

    pub fn get_oauth(&self) -> &OAuthProviders {
        &self.oauth
    }
}