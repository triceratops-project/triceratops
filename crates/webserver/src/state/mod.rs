use self::geo_ip::GeoIp;
use database::Database;
use deadpool_redis::Pool as RedisPool;
use oauth::OAuthProviders;
use redis::Cache;
use sea_orm::DatabaseConnection;
use std::sync::Arc;

mod database;
mod geo_ip;
mod oauth;
mod redis;

pub struct InternalAppState {
    pool: DatabaseConnection,
    cache: RedisPool,
    oauth: OAuthProviders,
    mm_db: Option<GeoIp>,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Self {
        let pool = Database::new().await;

        let cache = Cache::new().await;

        let oauth = OAuthProviders::default();

        let mm_db = GeoIp::new().await;
        let mm_db = Some(mm_db);

        Self {
            pool,
            cache,
            oauth,
            mm_db,
        }
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
