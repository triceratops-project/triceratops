use sea_orm_migration::prelude::*;

use crate::m20231122_135619_create_locations::Locations;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Nodes::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Nodes::Id)
                            .char_len(24)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Nodes::Name).string().not_null())
                    .col(ColumnDef::new(Nodes::Description).string())
                    .col(ColumnDef::new(Nodes::LocationId).string().not_null())
                    .col(ColumnDef::new(Nodes::ConnectionAddress).string().not_null())
                    .col(ColumnDef::new(Nodes::Secure).boolean().not_null())
                    .col(ColumnDef::new(Nodes::Token).string().not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_nodes_location_id")
                            .from(Nodes::Table, Nodes::LocationId)
                            .to(Locations::Table, Locations::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Nodes::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Nodes {
    Table,
    Id,
    Name,
    Description,
    LocationId,
    ConnectionAddress,
    Secure,
    Token,
}
