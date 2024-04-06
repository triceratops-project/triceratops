use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Locations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Locations::Id)
                            .char_len(8)
                            .not_null()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(Locations::ShortCode)
                            .char_len(2)
                            .not_null()
                            .unique_key(),
                    )
                    .col(ColumnDef::new(Locations::LongCode).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Locations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Locations {
    Table,
    Id,
    ShortCode,
    LongCode,
}
