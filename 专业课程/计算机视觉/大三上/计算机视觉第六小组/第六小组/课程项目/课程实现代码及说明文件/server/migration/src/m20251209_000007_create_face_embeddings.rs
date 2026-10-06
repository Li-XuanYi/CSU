use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // face_embeddings 存储人脸特征，用于人物聚类/相册
        manager
            .create_table(
                Table::create()
                    .table(FaceEmbeddings::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(FaceEmbeddings::Id)
                            .big_integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(FaceEmbeddings::PhotoId).integer().not_null())
                    .col(ColumnDef::new(FaceEmbeddings::Model).string().not_null())
                    // 使用 jsonb 存 embedding，方便在应用层解析；如需 pgvector 可后续迁移
                    .col(
                        ColumnDef::new(FaceEmbeddings::Embedding)
                            .json_binary()
                            .not_null(),
                    )
                    .col(ColumnDef::new(FaceEmbeddings::Bbox).json_binary())
                    .col(
                        ColumnDef::new(FaceEmbeddings::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        // 外键：关联照片，删除照片时级联删除人脸特征
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_face_embeddings_photo_id")
                    .from(FaceEmbeddings::Table, FaceEmbeddings::PhotoId)
                    .to(Photos::Table, Photos::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;

        // 查询优化
        manager
            .create_index(
                Index::create()
                    .name("idx_face_embeddings_photo_id")
                    .table(FaceEmbeddings::Table)
                    .col(FaceEmbeddings::PhotoId)
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_face_embeddings_model")
                    .table(FaceEmbeddings::Table)
                    .col(FaceEmbeddings::Model)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(FaceEmbeddings::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum FaceEmbeddings {
    Table,
    Id,
    PhotoId,
    Model,
    Embedding,
    Bbox,
    CreatedAt,
}

#[derive(DeriveIden)]
enum Photos {
    Table,
    Id,
}
