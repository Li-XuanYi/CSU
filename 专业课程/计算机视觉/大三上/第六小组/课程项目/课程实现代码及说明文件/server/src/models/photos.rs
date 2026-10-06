use salvo::oapi::ToSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub url: String,
    pub description: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub file_size: Option<i64>,
    pub mime_type: Option<String>,
    pub thumbnail_url: Option<String>,
    pub dominant_color: Option<String>,
    pub file_hash: Option<String>,
    pub processing_status: String,
    pub processed_at: Option<DateTime>,
    #[sea_orm(column_type = "JsonBinary")]
    pub ml_result: Option<JsonValue>,
    #[sea_orm(column_type = "JsonBinary")]
    pub ml_status: Option<JsonValue>,
    // EXIF fields
    pub date_time_original: Option<DateTime>,
    pub camera_make: Option<String>,
    pub camera_model: Option<String>,
    pub lens_model: Option<String>,
    pub focal_length: Option<String>,
    pub f_number: Option<String>,
    pub iso_speed: Option<i32>,
    pub exposure_time: Option<String>,
    pub gps_latitude: Option<f64>,
    pub gps_longitude: Option<f64>,
    pub gps_altitude: Option<f64>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::users::Entity",
        from = "Column::UserId",
        to = "super::users::Column::Id"
    )]
    User,
    #[sea_orm(has_many = "super::album_photos::Entity")]
    AlbumPhotos,
}

impl Related<super::users::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::User.def()
    }
}

// 通过中间表关联 albums
impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_photos::Relation::Albums.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_photos::Relation::Photos.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
