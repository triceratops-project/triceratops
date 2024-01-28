use crate::config::PostgresConfig;
use error_stack::{Context, Report, Result, ResultExt};
use sea_orm::{ConnectOptions, Database as SeaDatabase, DatabaseConnection};
use std::{fmt, time::Duration};

#[derive(Debug)]
pub struct DatabaseError;

impl fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Database module error")
    }
}

impl Context for DatabaseError {}

pub struct Database;

impl Database {
    pub async fn new(config: &PostgresConfig) -> Result<DatabaseConnection, DatabaseError> {
        let database_url = match config.password() {
            Some(pass) => format!(
                "postgres://{}:{}@{}:{}/{}",
                config.username(),
                pass,
                config.ip(),
                config.port(),
                config.schema()
            ),
            None => format!(
                "postgres://{}@{}:{}/{}",
                config.username(),
                config.ip(),
                config.port(),
                config.schema()
            ),
        };

        let mut opt = ConnectOptions::new(database_url);
        opt.max_connections(100)
            .min_connections(3)
            .connect_timeout(Duration::from_secs(3))
            .acquire_timeout(Duration::from_secs(3))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(120));

        let db = SeaDatabase::connect(opt)
            .await
            .map_err(Report::from)
            .attach_printable("Failed to connect to database")
            .change_context(DatabaseError)?;

        Ok(db)
    }
}
