pub use sea_orm_migration::prelude::*;

mod m20231120_235338_create_users;
mod m20231122_004821_create_servers;
mod m20231122_003448_create_nodes;
mod m20231122_135619_create_locations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20231120_235338_create_users::Migration),
            Box::new(m20231122_004821_create_servers::Migration),
            Box::new(m20231122_003448_create_nodes::Migration),
            Box::new(m20231122_135619_create_locations::Migration),
        ]
    }
}
