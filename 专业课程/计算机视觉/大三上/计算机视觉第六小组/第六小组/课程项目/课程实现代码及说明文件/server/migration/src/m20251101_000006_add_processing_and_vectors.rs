use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Enable pgvector extension (idempotent)
        manager
            .get_connection()
            .execute_unprepared("CREATE EXTENSION IF NOT EXISTS vector;")
            .await?;

        // Add processing/ML related columns to photos
        manager
            .alter_table(
                Table::alter()
                    .table(Photos::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Photos::ProcessingStatus)
                            .string_len(32)
                            .not_null()
                            .default("pending"),
                    )
                    .add_column_if_not_exists(ColumnDef::new(Photos::ProcessedAt).timestamp())
                    .add_column_if_not_exists(ColumnDef::new(Photos::MlResult).json_binary())
                    .to_owned(),
            )
            .await?;

        // Embedding table (pgvector)
        manager
            .get_connection()
            .execute_unprepared(
                r#"
                CREATE TABLE IF NOT EXISTS photo_embeddings (
                    id BIGSERIAL PRIMARY KEY,
                    photo_id INTEGER NOT NULL REFERENCES photos(id) ON DELETE CASCADE,
                    model VARCHAR NOT NULL,
                    modality VARCHAR NOT NULL,
                    embedding vector(768) NOT NULL,
                    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
                    UNIQUE(photo_id, model, modality)
                );
                "#,
            )
            .await?;

        // Helpful indexes for search
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX IF NOT EXISTS idx_photo_embeddings_model_modality ON photo_embeddings(model, modality);",
            )
            .await?;

        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX IF NOT EXISTS idx_photo_embeddings_vector ON photo_embeddings USING ivfflat (embedding vector_cosine_ops) WITH (lists = 100);",
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP TABLE IF EXISTS photo_embeddings;")
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Photos::Table)
                    .drop_column(Photos::ProcessingStatus)
                    .drop_column(Photos::ProcessedAt)
                    .drop_column(Photos::MlResult)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Photos {
    Table,
    ProcessingStatus,
    ProcessedAt,
    MlResult,
}
