use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(PhotoConversations::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(PhotoConversations::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(PhotoConversations::UserId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhotoConversations::PhotoId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhotoConversations::Model)
                            .string()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(PhotoConversations::Status)
                            .string_len(32)
                            .not_null(),
                    )
                    .col(ColumnDef::new(PhotoConversations::History).json_binary())
                    .col(ColumnDef::new(PhotoConversations::Error).text())
                    .col(
                        ColumnDef::new(PhotoConversations::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(PhotoConversations::UpdatedAt)
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
                    .name("idx_photo_conversations_user_status")
                    .table(PhotoConversations::Table)
                    .col(PhotoConversations::UserId)
                    .col(PhotoConversations::Status)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_photo_conversations_photo")
                    .table(PhotoConversations::Table)
                    .col(PhotoConversations::PhotoId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(PhotoConversations::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PhotoConversations {
    Table,
    Id,
    UserId,
    PhotoId,
    Model,
    Status,
    History,
    Error,
    CreatedAt,
    UpdatedAt,
}
