pub use sea_orm_migration::prelude::*;

mod m20231120_235338_create_users;
mod m20231122_003448_create_nodes;
mod m20231122_004821_create_servers;
mod m20231122_135619_create_locations;
mod m20231124_171543_create_nests;
mod m20231124_171550_create_eggs;
mod m20231124_210436_create_location_foreign_key;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231120_235338_create_users::Migration),
            Box::new(m20231122_003448_create_nodes::Migration),
            Box::new(m20231122_004821_create_servers::Migration),
            Box::new(m20231122_135619_create_locations::Migration),
            Box::new(m20231124_171543_create_nests::Migration),
            Box::new(m20231124_171550_create_eggs::Migration),
            Box::new(m20231124_210436_create_location_foreign_key::Migration),
        ]
    }
}
