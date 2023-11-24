use sea_orm_migration::prelude::*;

use crate::{m20231122_003448_create_nodes::Nodes, m20231122_135619_create_locations::Locations};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let location_id_fkey = TableForeignKey::new()
            .name("fk_nodes_location_id")
            .from_tbl(Nodes::Table)
            .from_col(Nodes::LocationId)
            .to_tbl(Locations::Table)
            .to_col(Locations::Id)
            .on_delete(ForeignKeyAction::Cascade)
            .on_update(ForeignKeyAction::Cascade)
            .to_owned();

        manager
            .alter_table(
                Table::alter()
                    .table(Nodes::Table)
                    .add_foreign_key(&location_id_fkey)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Post::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Post {
    Table,
    Id,
    Title,
    Text,
}
