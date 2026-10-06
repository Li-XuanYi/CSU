use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建 album_photos 中间表（多对多关系）
        manager
            .create_table(
                Table::create()
                    .table(AlbumPhotos::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(AlbumPhotos::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(AlbumPhotos::AlbumId).integer().not_null())
                    .col(ColumnDef::new(AlbumPhotos::PhotoId).integer().not_null())
                    .col(
                        ColumnDef::new(AlbumPhotos::CreatedAt)
                            .timestamp()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        // 创建外键：关联相册
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_album_photos_album_id")
                    .from(AlbumPhotos::Table, AlbumPhotos::AlbumId)
                    .to(Albums::Table, Albums::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建外键：关联照片
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_album_photos_photo_id")
                    .from(AlbumPhotos::Table, AlbumPhotos::PhotoId)
                    .to(Photos::Table, Photos::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 创建唯一索引：防止同一照片在同一相册中重复
        manager
            .create_index(
                Index::create()
                    .name("idx_album_photos_unique")
                    .table(AlbumPhotos::Table)
                    .col(AlbumPhotos::AlbumId)
                    .col(AlbumPhotos::PhotoId)
                    .unique()
                    .to_owned(),
            )
            .await?;

        // 创建索引：通过相册查询照片
        manager
            .create_index(
                Index::create()
                    .name("idx_album_photos_album_id")
                    .table(AlbumPhotos::Table)
                    .col(AlbumPhotos::AlbumId)
                    .to_owned(),
            )
            .await?;

        // 创建索引：通过照片查询相册
        manager
            .create_index(
                Index::create()
                    .name("idx_album_photos_photo_id")
                    .table(AlbumPhotos::Table)
                    .col(AlbumPhotos::PhotoId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(AlbumPhotos::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum AlbumPhotos {
    Table,
    Id,
    AlbumId,
    PhotoId,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Photos {
    Table,
    Id,
}

#[derive(DeriveIden)]
enum Albums {
    Table,
    Id,
}
