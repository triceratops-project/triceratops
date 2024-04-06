use sea_orm_migration::prelude::*;

use crate::{m20231120_235338_create_users::Users, m20231122_003448_create_nodes::Nodes};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Servers::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Servers::Id)
                            .char_len(24)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Servers::Name).string().not_null())
                    .col(ColumnDef::new(Servers::NodeId).char_len(24).not_null())
                    .col(ColumnDef::new(Servers::OwnerId).char_len(24).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_servers_node_id")
                            .from(Servers::Table, Servers::NodeId)
                            .to(Nodes::Table, Nodes::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_servers_owner_id")
                            .from(Servers::Table, Servers::OwnerId)
                            .to(Users::Table, Users::Id),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Servers::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Servers {
    Table,
    Id,
    Name,
    NodeId,
    OwnerId,
}
