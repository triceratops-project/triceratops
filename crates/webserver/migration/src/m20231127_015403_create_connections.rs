use sea_orm_migration::prelude::*;

use crate::m20231120_235338_create_users::Users;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Connections::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Connections::Id)
                            .char_len(24)
                            .not_null()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Connections::Provider).string().not_null())
                    .col(ColumnDef::new(Connections::UserId).char_len(24).not_null())
                    .col(ColumnDef::new(Connections::ExternalId).string().not_null())
                    .col(ColumnDef::new(Connections::Token).string().not_null())
                    .col(ColumnDef::new(Connections::RefreshToken).string())
                    .col(ColumnDef::new(Connections::Scopes).string().not_null())
                    .col(
                        ColumnDef::new(Connections::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Connections::ExpiresAt).timestamp())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_connections_user_id")
                            .from(Connections::Table, Connections::UserId)
                            .to(Users::Table, Users::Id)
                            .on_delete(ForeignKeyAction::Cascade)
                            .on_update(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Connections::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Connections {
    Table,
    Id,
    Provider,
    UserId,
    ExternalId,
    Token,
    RefreshToken,
    Scopes,
    CreatedAt,
    ExpiresAt,
}
