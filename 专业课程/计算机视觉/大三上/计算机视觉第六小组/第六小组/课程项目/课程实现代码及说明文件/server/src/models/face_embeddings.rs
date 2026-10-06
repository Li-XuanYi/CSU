use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "face_embeddings")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub photo_id: i32,
    pub model: String,
    #[sea_orm(column_type = "JsonBinary")]
    pub embedding: JsonValue,
    #[sea_orm(column_type = "JsonBinary")]
    pub bbox: Option<JsonValue>,
    pub created_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::photos::Entity",
        from = "Column::PhotoId",
        to = "super::photos::Column::Id"
    )]
    Photo,
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photo.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
