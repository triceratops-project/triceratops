use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};
use std::time::Duration;

pub struct Database;

impl Database {
    pub async fn new() -> DatabaseConnection {
        let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(120));

        SeaDatabase::connect(opt)
            .await
            .expect("Failed to connect to database")
    }
}
