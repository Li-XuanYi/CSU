use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "text_queries")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    pub user_id: i32,
    pub query: String,
    pub model: String,
    pub status: String,
    pub limit: Option<i32>,
    #[sea_orm(column_type = "JsonBinary")]
    pub embedding: Option<JsonValue>,
    #[sea_orm(column_type = "JsonBinary")]
    pub result: Option<JsonValue>,
    pub error: Option<String>,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
