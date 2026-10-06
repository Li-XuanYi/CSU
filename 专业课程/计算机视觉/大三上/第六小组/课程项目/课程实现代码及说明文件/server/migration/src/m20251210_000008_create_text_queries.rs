use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(TextQueries::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(TextQueries::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(TextQueries::UserId).integer().not_null())
                    .col(ColumnDef::new(TextQueries::Query).text().not_null())
                    .col(ColumnDef::new(TextQueries::Model).string().not_null())
                    .col(
                        ColumnDef::new(TextQueries::Status)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(ColumnDef::new(TextQueries::Limit).integer())
                    .col(ColumnDef::new(TextQueries::Embedding).json_binary())
                    .col(ColumnDef::new(TextQueries::Result).json_binary())
                    .col(ColumnDef::new(TextQueries::Error).text())
                    .col(
                        ColumnDef::new(TextQueries::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(TextQueries::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_text_queries_user_status")
                    .table(TextQueries::Table)
                    .col(TextQueries::UserId)
                    .col(TextQueries::Status)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(TextQueries::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum TextQueries {
    Table,
    Id,
    UserId,
    Query,
    Model,
    Status,
    Limit,
    Embedding,
    Result,
    Error,
    CreatedAt,
    UpdatedAt,
}
