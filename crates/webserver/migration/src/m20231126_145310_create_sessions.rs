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
                    .table(Sessions::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Sessions::Id)
                            .char_len(24)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Sessions::Token)
                            .char_len(128)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Sessions::UserId).char_len(24).not_null())
                    .col(ColumnDef::new(Sessions::IpAddress).string().not_null())
                    .col(ColumnDef::new(Sessions::ExpiresAt).timestamp())
                    .col(
                        ColumnDef::new(Sessions::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_sessions_user_id")
                            .from(Sessions::Table, Sessions::UserId)
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
            .drop_table(Table::drop().table(Sessions::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Sessions {
    Table,
    Id,
    Token,
    UserId,
    IpAddress,
    ExpiresAt,
    CreatedAt,
}
