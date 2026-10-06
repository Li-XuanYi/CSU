use salvo::oapi::ToSchema;
use sea_orm::Set;
use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum UserRole {
    Admin,
    User,
}

impl From<String> for UserRole {
    fn from(s: String) -> Self {
        match s.as_str() {
            "admin" => UserRole::Admin,
            _ => UserRole::User,
        }
    }
}

impl From<UserRole> for String {
    fn from(role: UserRole) -> Self {
        match role {
            UserRole::Admin => "admin".to_string(),
            UserRole::User => "user".to_string(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize, ToSchema)]
#[sea_orm(table_name = "users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    pub username: String,
    #[sea_orm(unique)]
    pub email: String,
    #[serde(skip_serializing)]
    pub password: String,
    pub role: String,
    #[serde(skip_serializing)]
    pub refresh_token: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
}

impl Model {
    pub fn get_role(&self) -> UserRole {
        UserRole::from(self.role.clone())
    }

    pub fn is_admin(&self) -> bool {
        matches!(self.get_role(), UserRole::Admin)
    }
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

/// 实现 ActiveModelBehavior 自动管理时间戳
#[async_trait::async_trait]
impl ActiveModelBehavior for ActiveModel {
    /// 在插入前设置时间戳
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        let now = chrono::Utc::now().naive_utc();

        if insert {
            // 新建记录时设置 created_at 和 updated_at
            self.created_at = Set(now);
            self.updated_at = Set(now);
        } else {
            // 更新记录时只更新 updated_at
            self.updated_at = Set(now);
        }

        Ok(self)
    }
}
