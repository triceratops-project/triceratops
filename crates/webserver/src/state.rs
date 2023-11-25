use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use std::{sync::Arc, time::Duration};

pub struct InternalAppState {
    pool: DatabaseConnection,
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

        Self { pool }
    }

    pub fn get_pool(&self) -> &DatabaseConnection {
        &self.pool
    }
}
