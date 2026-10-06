use salvo::oapi::ToSchema;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "albums")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_photo_id: Option<i32>,
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

// 通过中间表关联 photos
impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        super::album_photos::Relation::Photos.def()
    }

    fn via() -> Option<RelationDef> {
        Some(super::album_photos::Relation::Albums.def().rev())
    }
}

impl ActiveModelBehavior for ActiveModel {}
