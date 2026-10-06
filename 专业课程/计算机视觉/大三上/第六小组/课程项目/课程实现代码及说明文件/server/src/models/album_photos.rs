use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "album_photos")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub album_id: i32,
    pub photo_id: i32,
    pub created_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::albums::Entity",
        from = "Column::AlbumId",
        to = "super::albums::Column::Id"
    )]
    Albums,
    #[sea_orm(
        belongs_to = "super::photos::Entity",
        from = "Column::PhotoId",
        to = "super::photos::Column::Id"
    )]
    Photos,
}

impl Related<super::albums::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Albums.def()
    }
}

impl Related<super::photos::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Photos.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
