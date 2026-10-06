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
                    .table(Photos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Photos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Photos::UserId).integer().not_null())
                    .col(ColumnDef::new(Photos::Url).string().not_null())
                    .col(ColumnDef::new(Photos::Description).text())
                    .col(ColumnDef::new(Photos::Width).integer())
                    .col(ColumnDef::new(Photos::Height).integer())
                    .col(ColumnDef::new(Photos::FileSize).big_integer())
                    .col(ColumnDef::new(Photos::MimeType).string())
                    .col(ColumnDef::new(Photos::ThumbnailUrl).string())
                    .col(ColumnDef::new(Photos::DominantColor).string())
                    .col(ColumnDef::new(Photos::FileHash).string())
                    // EXIF 字段
                    .col(ColumnDef::new(Photos::DateTimeOriginal).timestamp())
                    .col(ColumnDef::new(Photos::CameraMake).string())
                    .col(ColumnDef::new(Photos::CameraModel).string())
                    .col(ColumnDef::new(Photos::LensModel).string())
                    .col(ColumnDef::new(Photos::FocalLength).string())
                    .col(ColumnDef::new(Photos::FNumber).string())
                    .col(ColumnDef::new(Photos::IsoSpeed).integer())
                    .col(ColumnDef::new(Photos::ExposureTime).string())
                    .col(ColumnDef::new(Photos::GpsLatitude).double())
                    .col(ColumnDef::new(Photos::GpsLongitude).double())
                    .col(ColumnDef::new(Photos::GpsAltitude).double())
                    .col(ColumnDef::new(Photos::CreatedAt).timestamp().not_null())
                    .col(ColumnDef::new(Photos::UpdatedAt).timestamp().not_null())
                    .to_owned(),
            )
            .await?;

        // 创建外键：照片属于某个用户
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_photos_user_id")
                    .from(Photos::Table, Photos::UserId)
                    .to(Users::Table, Users::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建索引：通过用户 ID 查询照片
        manager
            .create_index(
                Index::create()
                    .name("idx_photos_user_id")
                    .table(Photos::Table)
                    .col(Photos::UserId)
                    .to_owned(),
            )
            .await?;

        // 创建索引：通过文件哈希查询（用于去重）
        manager
            .create_index(
                Index::create()
                    .name("idx_photos_file_hash")
                    .table(Photos::Table)
                    .col(Photos::FileHash)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Photos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Photos {
    Table,
    Id,
    UserId,
    Url,
    Description,
    Width,
    Height,
    FileSize,
    MimeType,
    ThumbnailUrl,
    DominantColor,
    FileHash,
    // EXIF fields
    DateTimeOriginal,
    CameraMake,
    CameraModel,
    LensModel,
    FocalLength,
    FNumber,
    IsoSpeed,
    ExposureTime,
    GpsLatitude,
    GpsLongitude,
    GpsAltitude,
    CreatedAt,
    UpdatedAt,
}

#[derive(DeriveIden)]
enum Users {
    Table,
    Id,
}
