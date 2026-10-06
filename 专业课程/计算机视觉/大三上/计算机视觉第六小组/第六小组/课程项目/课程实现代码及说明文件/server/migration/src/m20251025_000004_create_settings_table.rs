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
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Settings::UserId).integer().not_null())
                    .col(ColumnDef::new(Settings::Key).string().not_null())
                    .col(ColumnDef::new(Settings::Value).text().not_null())
                    .col(ColumnDef::new(Settings::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Settings::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建外键：设置属于某个用户
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_settings_user_id")
                    .from(Settings::Table, Settings::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建唯一索引：同一用户的同一个设置键只能有一条记录
        manager
            .create_index(
                Index::create()
                    .name("idx_settings_user_key")
                    .table(Settings::Table)
                    .col(Settings::UserId)
                    .col(Settings::Key)
                    .unique()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Settings {
    Table,
    Id,
    UserId,
    Key,
    Value,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
