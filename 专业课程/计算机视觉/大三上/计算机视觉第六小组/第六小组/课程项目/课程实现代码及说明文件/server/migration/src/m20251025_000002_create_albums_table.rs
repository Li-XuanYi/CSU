use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建表
        manager
            .create_table(
                Table::create()
                    .table(Albums::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Albums::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Albums::UserId).integer().not_null())
                    .col(ColumnDef::new(Albums::Name).string().not_null())
                    .col(ColumnDef::new(Albums::Description).text())
                    .col(ColumnDef::new(Albums::CoverPhotoId).integer())
                    .col(ColumnDef::new(Albums::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Albums::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建外键
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_albums_user_id")
                    .from(Albums::Table, Albums::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建索引
        manager
            .create_index(
                Index::create()
                    .name("idx_albums_user_id")
                    .table(Albums::Table)
                    .col(Albums::UserId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Albums::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Albums {
    Table,
    Id,
    UserId,
    Name,
    Description,
    CoverPhotoId,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
