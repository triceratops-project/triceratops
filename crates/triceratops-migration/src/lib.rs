pub use sea_orm_migration::prelude::*;

mod m20231120_235338_create_users;
mod m20231122_003448_create_nodes;
mod m20231122_004821_create_servers;
mod m20231122_135619_create_locations;
mod m20231124_171543_create_collections;
mod m20231124_171550_create_containers;
mod m20231124_210436_create_location_foreign_key;
mod m20231126_145310_create_sessions;
mod m20231127_015403_create_connections;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231120_235338_create_users::Migration),
            Box::new(m20231122_003448_create_nodes::Migration),
            Box::new(m20231122_004821_create_servers::Migration),
            Box::new(m20231122_135619_create_locations::Migration),
            Box::new(m20231124_171543_create_collections::Migration),
            Box::new(m20231124_171550_create_containers::Migration),
            Box::new(m20231124_210436_create_location_foreign_key::Migration),
            Box::new(m20231126_145310_create_sessions::Migration),
            Box::new(m20231127_015403_create_connections::Migration),
        ]
    }
}
