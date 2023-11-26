use mobc::Pool;
use mobc_redis::{redis, RedisConnectionManager};
use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};

pub struct InternalAppState {
    pool: DatabaseConnection,
    redis: Pool<RedisConnectionManager>,
}

pub type AppState = Arc<InternalAppState>;

impl InternalAppState {
    pub async fn new() -> Self {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(120));

        let pool = Database::connect(opt)
            .await
            .expect("Failed to connect to database");

        let redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        let redis_connection = redis::Client::open(redis_url).expect("Failed to connect to redis");

        let redis_manager = RedisConnectionManager::new(redis_connection);

        let redis = Pool::builder()
            .get_timeout(Some(Duration::from_secs(3)))
            .max_lifetime(Some(Duration::from_secs(120)))
            .max_open(100)
            .max_idle(3)
            .build(redis_manager);

        Self { pool, redis }
    }

    pub fn get_pool(&self) -> &DatabaseConnection {
        &self.pool
    }

    pub fn get_redis(&self) -> &Pool<RedisConnectionManager> {
        &self.redis
    }
}
