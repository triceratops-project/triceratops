use sea_orm_migration::prelude::*;

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
}