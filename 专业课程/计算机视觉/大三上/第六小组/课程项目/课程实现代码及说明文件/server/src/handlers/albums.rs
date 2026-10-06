use salvo::oapi::ToSchema;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, Set,
};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::sync::Arc;

use crate::error::{AppError, Result};
use crate::handlers::photos::PhotoResponse;
use crate::models::{album_photos, albums, photos};

#[derive(Debug, Deserialize, ToSchema)]
pub struct CreateAlbumRequest {
    pub name: String,
    pub description: Option<String>,
    pub photo_ids: Option<Vec<i32>>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateAlbumRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub cover_photo_id: Option<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlbumResponse {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_photo_id: Option<i32>,
    pub cover_photo: Option<PhotoResponse>,
    pub created_at: String,
    pub updated_at: String,
}

impl From<albums::Model> for AlbumResponse {
    fn from(album: albums::Model) -> Self {
        Self {
            id: album.id,
            user_id: album.user_id,
            name: album.name,
            description: album.description,
            cover_photo_id: album.cover_photo_id,
            cover_photo: None,
            created_at: album.created_at.to_string(),
            updated_at: album.updated_at.to_string(),
        }
    }
}

impl From<(albums::Model, Vec<PhotoResponse>)> for AlbumDetailResponse {
    fn from((album, photos): (albums::Model, Vec<PhotoResponse>)) -> Self {
        let cover_photo = album
            .cover_photo_id
            .and_then(|cid| photos.iter().find(|p| p.id == cid).cloned())
            .or_else(|| photos.last().cloned());
        let cover_photo_id = album
            .cover_photo_id
            .or_else(|| cover_photo.as_ref().map(|p| p.id));
        Self {
            id: album.id,
            user_id: album.user_id,
            name: album.name,
            description: album.description,
            cover_photo_id,
            cover_photo,
            created_at: album.created_at.to_string(),
            updated_at: album.updated_at.to_string(),
            photos,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct AlbumDetailResponse {
    pub id: i32,
    pub user_id: i32,
    pub name: String,
    pub description: Option<String>,
    pub cover_photo_id: Option<i32>,
    pub cover_photo: Option<PhotoResponse>,
    pub created_at: String,
    pub updated_at: String,
    pub photos: Vec<PhotoResponse>,
}

/// 创建相册
#[endpoint(
    tags("Albums"),
    request_body = CreateAlbumRequest,
    responses(
        (status_code = 201, description = "Album created successfully", body = AlbumResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_album(
    body: JsonBody<CreateAlbumRequest>,
    depot: &mut Depot,
    res: &mut Response,
) -> Result<Json<AlbumResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let req = body.into_inner();

    let album = albums::ActiveModel {
        user_id: Set(user_id),
        name: Set(req.name),
        description: Set(req.description),
        cover_photo_id: Set(None),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let album = album
        .insert(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create album: {}", e)))?;

    let mut cover_photo: Option<PhotoResponse> = None;

    if let Some(photo_ids) = req.photo_ids {
        let mut unique_ids = Vec::new();
        let mut seen = HashSet::new();
        for pid in photo_ids {
            if seen.insert(pid) {
                unique_ids.push(pid);
            }
        }

        if !unique_ids.is_empty() {
            let owned_photos = photos::Entity::find()
                .filter(photos::Column::Id.is_in(unique_ids.clone()))
                .filter(photos::Column::UserId.eq(user_id))
                .all(db.as_ref())
                .await
                .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

            if owned_photos.len() != unique_ids.len() {
                return Err(AppError::Forbidden(
                    "包含不属于你的照片或照片不存在".to_string(),
                ));
            }

            cover_photo = owned_photos
                .iter()
                .cloned()
                .max_by_key(|p| p.created_at)
                .map(PhotoResponse::from);

            for pid in unique_ids {
                let album_photo = album_photos::ActiveModel {
                    album_id: Set(album.id),
                    photo_id: Set(pid),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };

                album_photo.insert(db.as_ref()).await.map_err(|e| {
                    AppError::Internal(format!("Failed to add photo to album: {}", e))
                })?;
            }
        }
    }

    let cover_photo_id = album
        .cover_photo_id
        .or_else(|| cover_photo.as_ref().map(|p| p.id));

    res.status_code(StatusCode::CREATED);
    Ok(Json(AlbumResponse {
        id: album.id,
        user_id: album.user_id,
        name: album.name,
        description: album.description,
        cover_photo_id,
        cover_photo,
        created_at: album.created_at.to_string(),
        updated_at: album.updated_at.to_string(),
    }))
}

/// 获取当前用户的所有相册
#[endpoint(
    tags("Albums"),
    responses(
        (status_code = 200, description = "List of albums", body = Vec<AlbumResponse>),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_albums(depot: &mut Depot) -> Result<Json<Vec<AlbumResponse>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let albums = albums::Entity::find()
        .filter(albums::Column::UserId.eq(user_id))
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let mut response: Vec<AlbumResponse> = Vec::new();
    for album in albums {
        let cover_photo = resolve_cover_photo(db.as_ref(), album.id, album.cover_photo_id).await?;
        let cover_photo_id = album
            .cover_photo_id
            .or_else(|| cover_photo.as_ref().map(|p| p.id));
        response.push(AlbumResponse {
            id: album.id,
            user_id: album.user_id,
            name: album.name.clone(),
            description: album.description.clone(),
            cover_photo_id,
            cover_photo,
            created_at: album.created_at.to_string(),
            updated_at: album.updated_at.to_string(),
        });
    }

    Ok(Json(response))
}

/// 获取单个相册详情
#[endpoint(
    tags("Albums"),
    parameters(
        ("id", description = "Album ID"),
    ),
    responses(
        (status_code = 200, description = "Album details", body = AlbumDetailResponse),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Album not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_album(id: PathParam<i32>, depot: &mut Depot) -> Result<Json<AlbumDetailResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let album = albums::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    let album_photos = album_photos::Entity::find()
        .filter(album_photos::Column::AlbumId.eq(*id))
        .order_by_asc(album_photos::Column::CreatedAt)
        .find_also_related(photos::Entity)
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let photos: Vec<PhotoResponse> = album_photos
        .into_iter()
        .filter_map(|(_, photo)| photo.map(PhotoResponse::from))
        .collect();

    Ok(Json(AlbumDetailResponse::from((album, photos))))
}

/// 更新相册
#[endpoint(
    tags("Albums"),
    parameters(
        ("id", description = "Album ID"),
    ),
    request_body = UpdateAlbumRequest,
    responses(
        (status_code = 200, description = "Album updated successfully", body = AlbumResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Album not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_album(
    id: PathParam<i32>,
    body: JsonBody<UpdateAlbumRequest>,
    depot: &mut Depot,
) -> Result<Json<AlbumResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    // Find album
    let album = albums::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    // Check ownership
    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    let req = body.into_inner();

    let mut album: albums::ActiveModel = album.into();

    if let Some(name) = req.name {
        album.name = Set(name);
    }

    if let Some(description) = req.description {
        album.description = Set(Some(description));
    }

    if let Some(cover_photo_id) = req.cover_photo_id {
        let cover_photo = photos::Entity::find_by_id(cover_photo_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
            .ok_or_else(|| AppError::NotFound("Cover photo not found".to_string()))?;

        if cover_photo.user_id != user_id {
            return Err(AppError::Forbidden(
                "Cover photo does not belong to user".to_string(),
            ));
        }

        let in_album = album_photos::Entity::find()
            .filter(album_photos::Column::AlbumId.eq(*id))
            .filter(album_photos::Column::PhotoId.eq(cover_photo_id))
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

        if in_album.is_none() {
            return Err(AppError::BadRequest(
                "Cover photo must be part of the album".to_string(),
            ));
        }

        album.cover_photo_id = Set(Some(cover_photo_id));
    }

    album.updated_at = Set(chrono::Utc::now().naive_utc());

    let album = album
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update album: {}", e)))?;

    let cover_photo = resolve_cover_photo(db.as_ref(), album.id, album.cover_photo_id).await?;
    let cover_photo_id = album
        .cover_photo_id
        .or_else(|| cover_photo.as_ref().map(|p| p.id));

    Ok(Json(AlbumResponse {
        id: album.id,
        user_id: album.user_id,
        name: album.name,
        description: album.description,
        cover_photo_id,
        cover_photo,
        created_at: album.created_at.to_string(),
        updated_at: album.updated_at.to_string(),
    }))
}

/// 向相册添加照片（相册视角）
#[endpoint(
    tags("Albums"),
    parameters(
        ("id", description = "Album ID"),
        ("photo_id", description = "Photo ID"),
    ),
    responses(
        (status_code = 200, description = "Photo added to album successfully"),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Album or photo not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_album_photo(
    id: PathParam<i32>,
    photo_id: PathParam<i32>,
    depot: &mut Depot,
) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let album = albums::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    let photo = photos::Entity::find_by_id(*photo_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;

    if photo.user_id != user_id {
        return Err(AppError::Forbidden(
            "Photo does not belong to user".to_string(),
        ));
    }

    let existing = album_photos::Entity::find()
        .filter(album_photos::Column::AlbumId.eq(*id))
        .filter(album_photos::Column::PhotoId.eq(*photo_id))
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(AppError::BadRequest(
            "Photo already in this album".to_string(),
        ));
    }

    let album_photo = album_photos::ActiveModel {
        album_id: Set(*id),
        photo_id: Set(*photo_id),
        created_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    album_photo
        .insert(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to add photo to album: {}", e)))?;

    Ok(StatusCode::OK)
}

/// 从相册移除照片（相册视角）
#[endpoint(
    tags("Albums"),
    parameters(
        ("id", description = "Album ID"),
        ("photo_id", description = "Photo ID"),
    ),
    responses(
        (status_code = 200, description = "Photo removed from album successfully"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Photo not found in album"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn remove_album_photo(
    id: PathParam<i32>,
    photo_id: PathParam<i32>,
    depot: &mut Depot,
) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let album = albums::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    let result = album_photos::Entity::delete_many()
        .filter(album_photos::Column::AlbumId.eq(*id))
        .filter(album_photos::Column::PhotoId.eq(*photo_id))
        .exec(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound(
            "Photo not found in this album".to_string(),
        ));
    }

    Ok(StatusCode::OK)
}

/// 删除相册
#[endpoint(
    tags("Albums"),
    parameters(
        ("id", description = "Album ID"),
    ),
    responses(
        (status_code = 200, description = "Album deleted successfully"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Album not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_album(id: PathParam<i32>, depot: &mut Depot) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    // Find album
    let album = albums::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    // Check ownership
    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    // Delete album (photos will be cascade deleted by database)
    albums::Entity::delete_by_id(*id)
        .exec(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete album: {}", e)))?;

    Ok(StatusCode::OK)
}

async fn resolve_cover_photo(
    db: &DatabaseConnection,
    album_id: i32,
    explicit_cover_id: Option<i32>,
) -> Result<Option<PhotoResponse>> {
    if let Some(cid) = explicit_cover_id {
        let cover = album_photos::Entity::find()
            .filter(album_photos::Column::AlbumId.eq(album_id))
            .filter(album_photos::Column::PhotoId.eq(cid))
            .find_also_related(photos::Entity)
            .one(db)
            .await
            .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

        if let Some((_, Some(photo))) = cover {
            return Ok(Some(PhotoResponse::from(photo)));
        }
    }

    let latest = album_photos::Entity::find()
        .filter(album_photos::Column::AlbumId.eq(album_id))
        .order_by_desc(album_photos::Column::CreatedAt)
        .find_also_related(photos::Entity)
        .one(db)
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    Ok(latest.and_then(|(_, p)| p).map(PhotoResponse::from))
}
