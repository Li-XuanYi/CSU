pub use sea_orm_migration::prelude::*;

mod m20250125_000001_create_users_table;
mod m20251025_000002_create_albums_table;
mod m20251025_000003_create_photos_table;
mod m20251025_000004_create_settings_table;
mod m20251025_000005_create_album_photos_junction;
mod m20251101_000006_add_processing_and_vectors;
mod m20251209_000007_create_face_embeddings;
mod m20251210_000008_create_text_queries;
mod m20251211_000009_add_ml_status;
mod m20251212_000010_create_photo_conversations;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20250125_000001_create_users_table::Migration),
            Box::new(m20251025_000002_create_albums_table::Migration),
            Box::new(m20251025_000003_create_photos_table::Migration),
            Box::new(m20251025_000004_create_settings_table::Migration),
            Box::new(m20251025_000005_create_album_photos_junction::Migration),
            Box::new(m20251101_000006_add_processing_and_vectors::Migration),
            Box::new(m20251209_000007_create_face_embeddings::Migration),
            Box::new(m20251210_000008_create_text_queries::Migration),
            Box::new(m20251211_000009_add_ml_status::Migration),
            Box::new(m20251212_000010_create_photo_conversations::Migration),
        ]
    }
}
