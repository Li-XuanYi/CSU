use redis::AsyncCommands;
use salvo::oapi::ToSchema;
use salvo::oapi::extract::{JsonBody, PathParam};
use salvo::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    JoinType, QueryFilter, QuerySelect, RelationTrait, Set, Statement, TransactionTrait, Value,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::{HashMap, HashSet};
use std::path::PathBuf;
use std::env;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;
use chrono::{Datelike, Utc};

use crate::config::Config;
use crate::error::{AppError, Result};
use crate::models::{
    album_photos, albums, face_embeddings, photo_conversations, photos, text_queries,
};
use crate::utils::{exif, image as img_utils, ml_status};

#[derive(Debug, Deserialize, ToSchema)]
pub struct UploadPhotoRequest {
    pub album_id: Option<i32>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct PhotoResponse {
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
    pub date_time_original: Option<String>,
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
    pub processing_status: String,
    pub processed_at: Option<String>,
    pub ml_result: Option<JsonValue>,
    pub ml_status: Option<JsonValue>,
}

impl From<photos::Model> for PhotoResponse {
    fn from(photo: photos::Model) -> Self {
        Self {
            id: photo.id,
            user_id: photo.user_id,
            url: photo.url,
            description: photo.description,
            width: photo.width,
            height: photo.height,
            file_size: photo.file_size,
            mime_type: photo.mime_type,
            thumbnail_url: photo.thumbnail_url,
            dominant_color: photo.dominant_color,
            date_time_original: photo.date_time_original.map(|dt| dt.to_string()),
            camera_make: photo.camera_make,
            camera_model: photo.camera_model,
            lens_model: photo.lens_model,
            focal_length: photo.focal_length,
            f_number: photo.f_number,
            iso_speed: photo.iso_speed,
            exposure_time: photo.exposure_time,
            gps_latitude: photo.gps_latitude,
            gps_longitude: photo.gps_longitude,
            gps_altitude: photo.gps_altitude,
            processing_status: photo.processing_status,
            processed_at: photo.processed_at.map(|dt| dt.to_string()),
            ml_result: photo.ml_result,
            ml_status: photo.ml_status,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdatePhotoMetadataRequest {
    pub description: Option<String>,
    pub dominant_color: Option<String>,
    /// RFC3339 string or "YYYY-MM-DD HH:MM:SS"
    pub date_time_original: Option<String>,
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
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateMlResultRequest {
    /// Embedding model name (default: config.processing.default_embedding_model)
    pub model: Option<String>,
    /// Modality of the embedding (e.g. "image", "text", "face")
    pub modality: Option<String>,
    /// Embedding values (must match configured dimension)
    pub embedding: Option<Vec<f32>>,
    /// Optional ML result payload (tags, caption, nsfw, boxes, etc.)
    pub ml_result: Option<JsonValue>,
    /// Optionally override processing status ("pending", "processing", "done", "failed")
    pub processing_status: Option<String>,
    /// If runner produced a new thumbnail, store its URL
    pub thumbnail_url: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FaceEmbeddingPayload {
    /// Embedding values for a single face (must match config.processing.face_embedding_dim)
    pub embedding: Vec<f32>,
    /// Optional model label (default: config.processing.default_face_embedding_model)
    pub model: Option<String>,
    /// Optional bounding box or landmark info from detector
    pub bbox: Option<JsonValue>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateFacesRequest {
    /// List of faces detected on this photo
    pub faces: Vec<FaceEmbeddingPayload>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct AutoPeopleAlbumRequest {
    /// Cosine distance threshold; lower is stricter (default: config.processing.face_cluster_threshold)
    pub threshold: Option<f32>,
    /// Minimum faces per cluster to create an album (default: config.processing.face_cluster_min_faces)
    pub min_faces_per_album: Option<usize>,
    /// Face embedding model label to use (default: config.processing.default_face_embedding_model)
    pub model: Option<String>,
    /// If true, do not create albums, only return clustering preview
    pub dry_run: Option<bool>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct PeopleAlbumResult {
    /// Newly created album id (None when dry_run)
    pub album_id: Option<i32>,
    pub name: String,
    pub face_count: usize,
    pub photo_ids: Vec<i32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct VectorSearchRequest {
    pub embedding: Vec<f32>,
    /// Optional override of model; falls back to config.processing.default_embedding_model
    pub model: Option<String>,
    /// Modality to query against, defaults to "image"
    pub modality: Option<String>,
    /// Max results, defaults to 20
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FusionVectorQuery {
    /// Embedding values for this branch
    pub embedding: Vec<f32>,
    /// Model label stored in photo_embeddings.model; default config.processing.default_embedding_model
    pub model: Option<String>,
    /// Modality for this branch, default "image"
    pub modality: Option<String>,
    /// Optional weight when fusing multiple branches, default 1.0
    pub weight: Option<f32>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct FusionVectorSearchRequest {
    /// Multiple embedding branches to fuse
    pub queries: Vec<FusionVectorQuery>,
    /// Result size, default 20
    pub limit: Option<i64>,
}

/// 文本搜索请求：服务端不做文本编码，需要先生成 embedding（由 runner 或客户端）
#[derive(Debug, Deserialize, ToSchema)]
pub struct TextSearchRequest {
    /// 原始查询文本（仅用于日志/调试）
    pub query: String,
    /// 已生成的文本 embedding（可选，缺失时会调用 PROCESSING_TEXT_EMBED_URL）
    pub embedding: Option<Vec<f32>>,
    /// 可选模型名，默认 config.processing.default_embedding_model
    pub model: Option<String>,
    /// 可选结果数量，默认 20
    pub limit: Option<i64>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TextSearchSubmitRequest {
    /// 查询文本（支持简单时间短语过滤，如“上周/本月/今天/last week”）
    pub query: String,
    /// 可选模型名，默认 config.processing.default_embedding_model
    pub model: Option<String>,
    /// 可选结果数量，默认 20
    pub limit: Option<i64>,
    /// 可选直接提交的 embedding；若缺失则走队列异步流程
    pub embedding: Option<Vec<f32>>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TextSearchStatusResponse {
    pub id: i64,
    pub status: String,
    pub result: Option<Vec<SimilarPhotoResult>>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct TextEmbedCallbackRequest {
    pub embedding: Vec<f32>,
    pub model: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LlavaChatRequest {
    pub query: String,
    pub model: Option<String>,
    pub temperature: Option<f32>,
    pub top_p: Option<f32>,
    pub num_beams: Option<u32>,
    pub max_new_tokens: Option<u32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct LlavaConversationResponse {
    pub id: i64,
    pub status: String,
    pub history: Option<JsonValue>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LlavaCallbackRequest {
    pub response: Option<String>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct ConversationMessage {
    role: String,
    content: String,
    created_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<JsonValue>,
}

#[derive(Debug, Clone)]
struct TextSearchFilters {
    date_range: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
    ocr_terms: Vec<String>,
    cleaned_query: String,
    time_ranges: Vec<(u8, u8)>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct SimilarPhotoResult {
    pub photo: PhotoResponse,
    /// Smaller distance means closer (cosine distance when embeddings are normalized)
    pub distance: f64,
}

#[derive(Debug, Serialize)]
struct ProcessingTask {
    photo_id: i32,
    user_id: i32,
    path: String,
    tasks: Vec<String>,
}

/// 上传照片
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Photo uploaded successfully", body = PhotoResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn upload_photo(req: &mut Request, depot: &mut Depot) -> Result<Json<PhotoResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<std::sync::Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;
    let redis = depot
        .obtain::<Arc<Mutex<redis::aio::ConnectionManager>>>()
        .map_err(|_| AppError::Internal("Redis connection not found in depot".to_string()))?;

    // Get user ID from JWT claims
    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    // Parse multipart form data
    let form_data = req
        .form_data()
        .await
        .map_err(|e| AppError::BadRequest(format!("Failed to parse form data: {}", e)))?;

    // Get metadata from form
    let album_ids: Vec<i32> = form_data
        .fields
        .get("album_ids")
        .and_then(|v| v.split(',').map(|s| s.trim().parse().ok()).collect())
        .unwrap_or_default();

    let description = form_data.fields.get("description").map(|s| s.to_string());

    let dominant_color = form_data
        .fields
        .get("dominant_color")
        .map(|s| s.to_string());

    // Get uploaded file
    let file = form_data
        .files
        .get("file")
        .ok_or_else(|| AppError::BadRequest("file is required".to_string()))?;

    // Validate file size
    let file_size = file.size();
    if file_size > config.storage.max_file_size as u64 {
        return Err(AppError::BadRequest(format!(
            "File size exceeds maximum allowed size of {} bytes",
            config.storage.max_file_size
        )));
    }

    // Validate file type
    let mime_type = mime_guess::from_path(&file.name().unwrap_or(""))
        .first()
        .ok_or_else(|| AppError::BadRequest("Could not determine file type".to_string()))?;

    if !mime_type.type_().as_str().eq("image") {
        return Err(AppError::BadRequest("File must be an image".to_string()));
    }

    // Generate unique filename
    let extension = std::path::Path::new(file.name().unwrap_or(""))
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    let filename = format!("{}.{}", Uuid::new_v4(), extension);

    // Create upload directories if they don't exist
    tokio::fs::create_dir_all(&config.storage.upload_dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create upload directory: {}", e)))?;
    tokio::fs::create_dir_all(&config.storage.thumbnail_dir)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to create thumbnail directory: {}", e)))?;

    // Save original file
    let original_path = PathBuf::from(&config.storage.upload_dir).join(&filename);
    tokio::fs::copy(file.path(), &original_path)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to save file: {}", e)))?;

    // Calculate file hash to check for duplicates
    let file_hash = img_utils::calculate_file_hash(&original_path)?;

    // Check if a photo with this hash already exists for this user
    let existing_photo = photos::Entity::find()
        .filter(photos::Column::FileHash.eq(&file_hash))
        .filter(photos::Column::UserId.eq(user_id))
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if let Some(existing) = existing_photo {
        // Photo already exists, delete the newly uploaded file and return existing photo
        let _ = tokio::fs::remove_file(&original_path).await;

        // Add to albums if album_ids provided
        for album_id in album_ids {
            let album = albums::Entity::find_by_id(album_id)
                .one(db.as_ref())
                .await
                .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

            if let Some(album) = album {
                if album.user_id == user_id {
                    // Check if association already exists
                    let existing_assoc = album_photos::Entity::find()
                        .filter(album_photos::Column::AlbumId.eq(album_id))
                        .filter(album_photos::Column::PhotoId.eq(existing.id))
                        .one(db.as_ref())
                        .await
                        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

                    if existing_assoc.is_none() {
                        let album_photo = album_photos::ActiveModel {
                            album_id: Set(album_id),
                            photo_id: Set(existing.id),
                            created_at: Set(chrono::Utc::now().naive_utc()),
                            ..Default::default()
                        };

                        album_photo.insert(db.as_ref()).await.map_err(|e| {
                            AppError::Internal(format!("Failed to add photo to album: {}", e))
                        })?;
                    }
                }
            }
        }

        return Ok(Json(PhotoResponse::from(existing)));
    }

    // Photo is new, continue with normal upload process
    // Extract image info (fast operation)
    let image_info = img_utils::get_image_info(&original_path)?;

    // Extract EXIF data (fast operation)
    let exif_data = exif::extract_exif(&original_path)?;

    let tasks = vec![
        "thumbnail".to_string(),
        "clip".to_string(),
        "face".to_string(),
        "ocr".to_string(),
    ];
    let ml_status = ml_status::build_ml_status(&tasks);

    // Create photo record with thumbnail_url as None (will be updated by background task)
    let photo = photos::ActiveModel {
        user_id: Set(user_id),
        url: Set(format!("/uploads/original/{}", filename)),
        description: Set(description),
        width: Set(Some(image_info.width as i32)),
        height: Set(Some(image_info.height as i32)),
        file_size: Set(Some(image_info.file_size as i64)),
        mime_type: Set(Some(mime_type.to_string())),
        thumbnail_url: Set(None), // Will be set by background task
        dominant_color: Set(dominant_color), // From frontend canvas extraction
        file_hash: Set(Some(file_hash.clone())), // Store file hash for deduplication
        date_time_original: Set(exif_data.date_time_original),
        camera_make: Set(exif_data.make),
        camera_model: Set(exif_data.model),
        lens_model: Set(exif_data.lens_model),
        focal_length: Set(exif_data.focal_length),
        f_number: Set(exif_data.f_number),
        iso_speed: Set(exif_data.iso_speed.map(|v| v as i32)),
        exposure_time: Set(exif_data.exposure_time),
        gps_latitude: Set(exif_data.gps_latitude),
        gps_longitude: Set(exif_data.gps_longitude),
        gps_altitude: Set(exif_data.gps_altitude),
        processing_status: Set("pending".to_string()),
        processed_at: Set(None),
        ml_result: Set(None),
        ml_status: Set(Some(ml_status)),
        created_at: Set(chrono::Utc::now().naive_utc()),
        updated_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    };

    let photo = photo
        .insert(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to save photo to database: {}", e)))?;

    // Spawn background task to generate thumbnail
    let photo_id = photo.id;
    let db_clone = Arc::clone(&db);
    let config_clone = Arc::clone(&config);
    let original_path_clone = original_path.clone();
    let filename_clone = filename.clone();

    tokio::spawn(async move {
        if let Err(e) = generate_thumbnail_background(
            photo_id,
            original_path_clone,
            filename_clone,
            db_clone,
            config_clone,
        )
        .await
        {
            eprintln!("Failed to generate thumbnail for photo {}: {}", photo_id, e);
        }
    });

    // Add photo to albums if album_ids provided
    for album_id in album_ids {
        // Verify album exists and belongs to user
        let album = albums::Entity::find_by_id(album_id)
            .one(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

        if let Some(album) = album {
            if album.user_id == user_id {
                // Create album-photo association
                let album_photo = album_photos::ActiveModel {
                    album_id: Set(album_id),
                    photo_id: Set(photo.id),
                    created_at: Set(chrono::Utc::now().naive_utc()),
                    ..Default::default()
                };

                album_photo.insert(db.as_ref()).await.map_err(|e| {
                    AppError::Internal(format!("Failed to add photo to album: {}", e))
                })?;
            }
        }
    }

    let task = ProcessingTask {
        photo_id: photo.id,
        user_id,
        path: original_path.to_string_lossy().to_string(),
        tasks,
    };

    enqueue_processing_task(redis.clone(), &config.redis.queue_name, task).await?;

    Ok(Json(PhotoResponse::from(photo)))
}

/// 获取照片列表（按相册）
#[endpoint(
    tags("Photos"),
    parameters(
        ("album_id", description = "Album ID"),
    ),
    responses(
        (status_code = 200, description = "List of photos", body = Vec<PhotoResponse>),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_photos(
    album_id: PathParam<i32>,
    depot: &mut Depot,
) -> Result<Json<Vec<PhotoResponse>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    // 通过中间表查询相册中的照片
    let album_photos = album_photos::Entity::find()
        .filter(album_photos::Column::AlbumId.eq(*album_id))
        .find_also_related(photos::Entity)
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let photos: Vec<PhotoResponse> = album_photos
        .into_iter()
        .filter_map(|(_, photo)| photo.map(PhotoResponse::from))
        .collect();

    Ok(Json(photos))
}

/// 获取当前用户的所有照片
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "List of all user photos", body = Vec<PhotoResponse>),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_my_photos(depot: &mut Depot) -> Result<Json<Vec<PhotoResponse>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let photos = photos::Entity::find()
        .filter(photos::Column::UserId.eq(user_id))
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let response: Vec<PhotoResponse> = photos.into_iter().map(PhotoResponse::from).collect();

    Ok(Json(response))
}

/// 获取单个照片详情
#[endpoint(
    tags("Photos"),
    parameters(
        ("id", description = "Photo ID"),
    ),
    responses(
        (status_code = 200, description = "Photo details", body = PhotoResponse),
        (status_code = 404, description = "Photo not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_photo(id: PathParam<i32>, depot: &mut Depot) -> Result<Json<PhotoResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;
    Ok(Json(PhotoResponse::from(photo)))
}

/// 更新照片的元信息
#[endpoint(
    tags("Photos"),
    parameters(
        ("id", description = "Photo ID"),
    ),
    request_body = UpdatePhotoMetadataRequest,
    responses(
        (status_code = 200, description = "Photo metadata updated", body = PhotoResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Photo not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_photo_metadata(
    id: PathParam<i32>,
    body: JsonBody<UpdatePhotoMetadataRequest>,
    depot: &mut Depot,
) -> Result<Json<PhotoResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;

    if photo.user_id != user_id {
        return Err(AppError::Forbidden(
            "Photo does not belong to user".to_string(),
        ));
    }

    let payload = body.into_inner();
    let mut active: photos::ActiveModel = photo.into();

    if let Some(description) = payload.description {
        active.description = Set(Some(description));
    }

    if let Some(color) = payload.dominant_color {
        active.dominant_color = Set(Some(color));
    }

    if let Some(dt) = payload.date_time_original {
        let parsed = parse_datetime(&dt)?;
        active.date_time_original = Set(Some(parsed));
    }

    if let Some(make) = payload.camera_make {
        active.camera_make = Set(Some(make));
    }

    if let Some(model) = payload.camera_model {
        active.camera_model = Set(Some(model));
    }

    if let Some(lens_model) = payload.lens_model {
        active.lens_model = Set(Some(lens_model));
    }

    if let Some(focal_length) = payload.focal_length {
        active.focal_length = Set(Some(focal_length));
    }

    if let Some(f_number) = payload.f_number {
        active.f_number = Set(Some(f_number));
    }

    if let Some(iso_speed) = payload.iso_speed {
        active.iso_speed = Set(Some(iso_speed));
    }

    if let Some(exposure_time) = payload.exposure_time {
        active.exposure_time = Set(Some(exposure_time));
    }

    if let Some(lat) = payload.gps_latitude {
        active.gps_latitude = Set(Some(lat));
    }

    if let Some(lon) = payload.gps_longitude {
        active.gps_longitude = Set(Some(lon));
    }

    if let Some(alt) = payload.gps_altitude {
        active.gps_altitude = Set(Some(alt));
    }

    active.updated_at = Set(chrono::Utc::now().naive_utc());

    let photo = active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update photo metadata: {}", e)))?;

    Ok(Json(PhotoResponse::from(photo)))
}

/// 删除照片
#[endpoint(
    tags("Photos"),
    parameters(
        ("id", description = "Photo ID"),
    ),
    responses(
        (status_code = 200, description = "Photo deleted successfully"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Photo not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_photo(id: PathParam<i32>, depot: &mut Depot) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<std::sync::Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    // Find photo
    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;

    // Check ownership
    if photo.user_id != user_id {
        return Err(AppError::Forbidden(
            "Photo does not belong to user".to_string(),
        ));
    }

    // Delete files
    let filename = photo.url.split('/').last().unwrap_or("");
    let original_path = PathBuf::from(&config.storage.upload_dir).join(filename);
    let thumbnail_path = PathBuf::from(&config.storage.thumbnail_dir).join(filename);

    // Ignore errors when deleting files (they might not exist)
    let _ = tokio::fs::remove_file(original_path).await;
    let _ = tokio::fs::remove_file(thumbnail_path).await;

    // Delete from database
    photos::Entity::delete_by_id(*id)
        .exec(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to delete photo: {}", e)))?;

    Ok(StatusCode::OK)
}

/// 添加照片到相册
#[endpoint(
    tags("Photos"),
    parameters(
        ("photo_id", description = "Photo ID"),
        ("album_id", description = "Album ID"),
    ),
    responses(
        (status_code = 200, description = "Photo added to album successfully"),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Photo or album not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_photo_to_album(
    photo_id: PathParam<i32>,
    album_id: PathParam<i32>,
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

    // Verify photo exists and belongs to user
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

    // Verify album exists and belongs to user
    let album = albums::Entity::find_by_id(*album_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    // Check if association already exists
    let existing = album_photos::Entity::find()
        .filter(album_photos::Column::AlbumId.eq(*album_id))
        .filter(album_photos::Column::PhotoId.eq(*photo_id))
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if existing.is_some() {
        return Err(AppError::BadRequest("Photo already in album".to_string()));
    }

    // Create association
    let album_photo = album_photos::ActiveModel {
        album_id: Set(*album_id),
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

/// 从相册移除照片
#[endpoint(
    tags("Photos"),
    parameters(
        ("photo_id", description = "Photo ID"),
        ("album_id", description = "Album ID"),
    ),
    responses(
        (status_code = 200, description = "Photo removed from album successfully"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 403, description = "Forbidden"),
        (status_code = 404, description = "Association not found"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn remove_photo_from_album(
    photo_id: PathParam<i32>,
    album_id: PathParam<i32>,
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

    // Verify album belongs to user
    let album = albums::Entity::find_by_id(*album_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Album not found".to_string()))?;

    if album.user_id != user_id {
        return Err(AppError::Forbidden(
            "Album does not belong to user".to_string(),
        ));
    }

    // Delete association
    let result = album_photos::Entity::delete_many()
        .filter(album_photos::Column::AlbumId.eq(*album_id))
        .filter(album_photos::Column::PhotoId.eq(*photo_id))
        .exec(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if result.rows_affected == 0 {
        return Err(AppError::NotFound("Photo not in this album".to_string()));
    }

    Ok(StatusCode::OK)
}

/// Internal callback for ML runner to persist embeddings and ML results
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Updated successfully", body = PhotoResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Photo not found"),
        (status_code = 500, description = "Internal server error")
    )
)]
pub async fn update_ml_result(
    id: PathParam<i32>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<PhotoResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let token = req
        .headers()
        .get("x-internal-token")
        .and_then(|v| v.to_str().ok());

    if token != Some(config.processing.internal_token.as_str()) {
        return Err(AppError::Unauthorized("Invalid internal token".to_string()));
    }

    let payload: UpdateMlResultRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;
    let existing_ml = photo.ml_result.clone();
    let existing_status = photo.ml_status.clone();

    if let Some(embedding) = payload.embedding.as_ref() {
        if embedding.len() != config.processing.embedding_dim {
            return Err(AppError::BadRequest(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                config.processing.embedding_dim,
                embedding.len()
            )));
        }

        let vector_literal = to_vector_literal(embedding);
        let model = payload
            .model
            .clone()
            .unwrap_or_else(|| config.processing.default_embedding_model.clone());
        let modality = payload
            .modality
            .clone()
            .unwrap_or_else(|| "image".to_string());

        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            r#"
            INSERT INTO photo_embeddings (photo_id, model, modality, embedding)
            VALUES ($1, $2, $3, $4::vector)
            ON CONFLICT (photo_id, model, modality)
            DO UPDATE SET embedding = EXCLUDED.embedding
            "#,
            vec![
                Value::from(*id),
                Value::from(model),
                Value::from(modality),
                Value::from(vector_literal),
            ],
        );

        db.execute(stmt)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to upsert embedding: {}", e)))?;
    }

    let mut active: photos::ActiveModel = photo.into();
    if let Some(ref ml_result) = payload.ml_result {
        let merged = merge_ml_result(existing_ml, ml_result.clone());
        active.ml_result = Set(Some(merged));
    }
    if let Some(ref url) = payload.thumbnail_url {
        active.thumbnail_url = Set(Some(url.clone()));
    }

    let mut status_updates: Vec<(&str, &str)> = Vec::new();
    if let Some(ref embedding) = payload.embedding {
        let modality = payload
            .modality
            .clone()
            .unwrap_or_else(|| "image".to_string());
        if modality == "image" {
            status_updates.push(("clip", "done"));
        } else if modality == "caption" {
            status_updates.push(("caption", "done"));
        }
        if embedding.is_empty() {
            status_updates.clear();
        }
    }
    if let Some(ref ml_result) = payload.ml_result {
        if ml_result.get("ocr_text").is_some() || ml_result.get("ocr").is_some() {
            status_updates.push(("ocr", "done"));
        }
    }
    if payload.thumbnail_url.is_some() {
        status_updates.push(("thumbnail", "done"));
    }
    if !status_updates.is_empty() {
        active.ml_status = Set(Some(ml_status::merge_ml_status(
            existing_status,
            &status_updates,
        )));
    }

    active.processing_status = Set(payload
        .processing_status
        .unwrap_or_else(|| "done".to_string()));
    active.processed_at = Set(Some(chrono::Utc::now().naive_utc()));
    active.updated_at = Set(chrono::Utc::now().naive_utc());

    let photo = active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update photo metadata: {}", e)))?;

    Ok(Json(PhotoResponse::from(photo)))
}

/// Internal callback for ML runner to persist face embeddings (multiple faces per photo)
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Faces updated successfully"),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Photo not found"),
        (status_code = 500, description = "Internal server error")
    )
)]
pub async fn update_faces(
    id: PathParam<i32>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let token = req
        .headers()
        .get("x-internal-token")
        .and_then(|v| v.to_str().ok());

    if token != Some(config.processing.internal_token.as_str()) {
        return Err(AppError::Unauthorized("Invalid internal token".to_string()));
    }

    let payload: UpdateFacesRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;
    let existing_status = photo.ml_status.clone();

    if payload.faces.is_empty() {
        let mut photo: photos::ActiveModel = photo.into();
        photo.ml_status = Set(Some(ml_status::merge_ml_status(
            existing_status,
            &[("face", "done")],
        )));
        photo.updated_at = Set(chrono::Utc::now().naive_utc());
        photo
            .update(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update ml_status: {}", e)))?;
        return Ok(StatusCode::OK);
    }

    let mut faces_to_insert: Vec<(String, JsonValue, Option<JsonValue>)> = Vec::new();
    let mut models: Vec<String> = Vec::new();
    let dim = config.processing.face_embedding_dim;

    for face in payload.faces {
        if face.embedding.len() != dim {
            return Err(AppError::BadRequest(format!(
                "Face embedding dimension mismatch: expected {}, got {}",
                dim,
                face.embedding.len()
            )));
        }
        let model = face
            .model
            .unwrap_or_else(|| config.processing.default_face_embedding_model.clone());
        let emb_json = serde_json::to_value(face.embedding).map_err(|e| {
            AppError::Internal(format!("Failed to serialize embedding to JSON: {}", e))
        })?;
        faces_to_insert.push((model.clone(), emb_json, face.bbox));
        models.push(model);
    }

    // Remove existing embeddings for this photo+model combination to avoid duplicates
    let unique_models: Vec<String> = {
        let mut s = HashSet::new();
        models.into_iter().filter(|m| s.insert(m.clone())).collect()
    };

    let txn = db
        .begin()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to start transaction: {}", e)))?;

    if !unique_models.is_empty() {
        face_embeddings::Entity::delete_many()
            .filter(face_embeddings::Column::PhotoId.eq(*id))
            .filter(face_embeddings::Column::Model.is_in(unique_models.clone()))
            .exec(&txn)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to delete old faces: {}", e)))?;
    }

    let now = chrono::Utc::now();
    for (model, emb, bbox) in faces_to_insert {
        let active = face_embeddings::ActiveModel {
            id: Default::default(),
            photo_id: Set(*id),
            model: Set(model),
            embedding: Set(emb),
            bbox: Set(bbox),
            created_at: Set(now.into()),
        };

        active
            .insert(&txn)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to insert face embedding: {}", e)))?;
    }

    txn.commit()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to commit faces: {}", e)))?;

    let mut photo: photos::ActiveModel = photo.into();
    photo.ml_status = Set(Some(ml_status::merge_ml_status(
        existing_status,
        &[("face", "done")],
    )));
    photo.updated_at = Set(chrono::Utc::now().naive_utc());
    photo
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update ml_status: {}", e)))?;

    if let Ok(flag) = depot.obtain::<Arc<AtomicBool>>() {
        flag.store(true, Ordering::SeqCst);
    }

    Ok(StatusCode::OK)
}

/// 将人脸向量聚类并为每个簇创建相册（或仅预览）
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Clustered and created albums", body = Vec<PeopleAlbumResult>),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(("bearer_auth" = []))
)]
pub async fn auto_people_albums(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<Vec<PeopleAlbumResult>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: AutoPeopleAlbumRequest =
        req.parse_json()
            .await
            .unwrap_or_else(|_| AutoPeopleAlbumRequest {
                threshold: None,
                min_faces_per_album: None,
                model: None,
                dry_run: None,
            });

    let threshold = payload
        .threshold
        .unwrap_or(config.processing.face_cluster_threshold);
    let min_faces = payload
        .min_faces_per_album
        .unwrap_or(config.processing.face_cluster_min_faces);
    let model = payload
        .model
        .unwrap_or_else(|| config.processing.default_face_embedding_model.clone());
    let dry_run = payload.dry_run.unwrap_or(false);

    let responses = generate_people_albums(
        db.as_ref(),
        user_id,
        threshold,
        min_faces,
        &model,
        dry_run,
    )
    .await?;

    Ok(Json(responses))
}

/// Vector search by provided embedding (text or image embedding)
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Search results", body = Vec<SimilarPhotoResult>),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn search_by_vector(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<Vec<SimilarPhotoResult>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: VectorSearchRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    if payload.embedding.len() != config.processing.embedding_dim {
        return Err(AppError::BadRequest(format!(
            "Embedding dimension mismatch: expected {}, got {}",
            config.processing.embedding_dim,
            payload.embedding.len()
        )));
    }

    let vector_literal = to_vector_literal(&payload.embedding);
    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| config.processing.default_embedding_model.clone());
    let modality = payload
        .modality
        .clone()
        .unwrap_or_else(|| "image".to_string());
    // Text embedding should still search against image embeddings produced by clip_runner
    let target_modality = if modality == "text" {
        "image".to_string()
    } else {
        modality.clone()
    };
    let limit = payload.limit.unwrap_or(20);

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT pe.photo_id, pe.embedding <=> $1::vector AS distance
        FROM photo_embeddings pe
        WHERE pe.model = $2
          AND pe.modality = $3
          AND EXISTS (
            SELECT 1 FROM photos p WHERE p.id = pe.photo_id AND p.user_id = $4
          )
        ORDER BY distance ASC
        LIMIT $5
        "#,
        vec![
            Value::from(vector_literal),
            Value::from(model),
            Value::from(target_modality),
            Value::from(user_id),
            Value::from(limit),
        ],
    );

    let rows = db
        .query_all(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Search query failed: {}", e)))?;

    let mut ordered: Vec<(i32, f64)> = Vec::new();
    for row in rows {
        let photo_id: i32 = row
            .try_get("", "photo_id")
            .map_err(|e| AppError::Internal(format!("Failed to decode photo_id: {}", e)))?;
        let distance: f64 = row
            .try_get("", "distance")
            .map_err(|e| AppError::Internal(format!("Failed to decode distance: {}", e)))?;
        ordered.push((photo_id, distance));
    }

    if ordered.is_empty() {
        return Ok(Json(vec![]));
    }

    let photo_ids: Vec<i32> = ordered.iter().map(|(id, _)| *id).collect();
    let models = photos::Entity::find()
        .filter(photos::Column::Id.is_in(photo_ids.clone()))
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let mut map: HashMap<i32, photos::Model> = models.into_iter().map(|m| (m.id, m)).collect();

    let mut results: Vec<SimilarPhotoResult> = Vec::new();
    for (pid, distance) in ordered {
        if let Some(photo) = map.remove(&pid) {
            results.push(SimilarPhotoResult {
                photo: PhotoResponse::from(photo),
                distance,
            });
        }
    }

    Ok(Json(results))
}

/// 融合多条向量分支（不同 model/modality）的搜图
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Search results", body = Vec<SimilarPhotoResult>),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn search_by_fusion(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<Vec<SimilarPhotoResult>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: FusionVectorSearchRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    if payload.queries.is_empty() {
        return Err(AppError::BadRequest("queries is required".to_string()));
    }

    let limit = payload.limit.unwrap_or(20);
    let search_limit = (limit * 3).max(20); // 多拿一些用于融合

    let mut agg: HashMap<i32, (PhotoResponse, f64, f64)> = HashMap::new();

    for q in payload.queries {
        if q.embedding.len() != config.processing.embedding_dim {
            return Err(AppError::BadRequest(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                config.processing.embedding_dim,
                q.embedding.len()
            )));
        }

        let model = q
            .model
            .clone()
            .unwrap_or_else(|| config.processing.default_embedding_model.clone());
        let modality = q.modality.clone().unwrap_or_else(|| "image".to_string());
        let weight = q.weight.unwrap_or(1.0).max(0.0) as f64;
        if weight == 0.0 {
            continue;
        }

        let results =
            run_vector_search(
                db.as_ref(),
                user_id,
                &model,
                &modality,
                &q.embedding,
                search_limit,
                None,
                &[],
            )
                .await?;

        for item in results {
            let entry = agg
                .entry(item.photo.id)
                .or_insert((item.photo.clone(), 0.0f64, 0.0f64));
            entry.1 += weight * item.distance;
            entry.2 += weight;
        }
    }

    let mut fused: Vec<SimilarPhotoResult> = agg
        .into_iter()
        .filter_map(|(_, (photo, dist_sum, w_sum))| {
            if w_sum == 0.0 {
                None
            } else {
                Some(SimilarPhotoResult {
                    photo,
                    distance: dist_sum / w_sum,
                })
            }
        })
        .collect();

    fused.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(max_dist) = max_search_distance() {
        fused.retain(|item| item.distance <= max_dist);
    }
    fused.truncate(limit as usize);

    Ok(Json(fused))
}

/// 文本搜图：需要调用方先生成文本 embedding 再调用
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Search results", body = TextSearchStatusResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn search_by_text(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<TextSearchStatusResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;
    let redis = depot
        .obtain::<Arc<Mutex<redis::aio::ConnectionManager>>>()
        .map_err(|_| AppError::Internal("Redis connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: TextSearchSubmitRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| config.processing.default_embedding_model.clone());
    let limit = payload.limit.unwrap_or(20);
    let query = payload.query.clone();
    let filters = parse_text_search_filters(&query, chrono::Utc::now());

    // 创建记录 pending
    let now = chrono::Utc::now();
    let record = text_queries::ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        query: Set(query.clone()),
        model: Set(model.clone()),
        status: Set("pending".to_string()),
        limit: Set(Some(limit as i32)),
        embedding: Set(None),
        result: Set(None),
        error: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
    .insert(db.as_ref())
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create text query: {}", e)))?;

    // 优先同步路径（传入 embedding 或配置了文本服务）
    let embedding_opt = if let Some(vec) = payload.embedding {
        Some(vec)
    } else if let Some(url) = config.processing.text_embed_url.clone() {
        Some(fetch_text_embedding(&payload.query, &model, &url).await?)
    } else {
        None
    };

    if let Some(embedding) = embedding_opt {
        if embedding.len() != config.processing.embedding_dim {
            return Err(AppError::BadRequest(format!(
                "Embedding dimension mismatch: expected {}, got {}",
                config.processing.embedding_dim,
                embedding.len()
            )));
        }

        let results =
            run_text_fusion_search(db.as_ref(), user_id, &model, &embedding, limit, &filters).await?;

        let now = chrono::Utc::now();
        let mut active: text_queries::ActiveModel = record.into();
        active.status = Set("done".to_string());
        active.embedding = Set(Some(serde_json::to_value(&embedding).map_err(|e| {
            AppError::Internal(format!("Failed to serialize embedding: {}", e))
        })?));
        active.result = Set(Some(serde_json::to_value(&results).map_err(|e| {
            AppError::Internal(format!("Failed to serialize result: {}", e))
        })?));
        active.updated_at = Set(now.into());
        let saved = active
            .update(db.as_ref())
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update text query: {}", e)))?;

        return Ok(Json(TextSearchStatusResponse {
            id: saved.id,
            status: saved.status,
            result: Some(results),
            error: saved.error,
        }));
    }

    // 异步路径：入队等待 runner 生成 embedding
    enqueue_text_task(
        redis.clone(),
        &config.processing.text_embed_queue,
        record.id,
        &query,
        &model,
        limit,
    )
    .await?;

    Ok(Json(TextSearchStatusResponse {
        id: record.id,
        status: "pending".to_string(),
        result: None,
        error: None,
    }))
}

/// 查询文本搜索状态/结果
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Status/result", body = TextSearchStatusResponse),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_text_search_status(
    id: PathParam<i64>,
    depot: &mut Depot,
) -> Result<Json<TextSearchStatusResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let record = text_queries::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Query not found".to_string()))?;

    if record.user_id != user_id {
        return Err(AppError::Forbidden("Not your query".to_string()));
    }

    let result = match record.result {
        Some(ref val) => serde_json::from_value(val.clone()).ok(),
        None => None,
    };

    Ok(Json(TextSearchStatusResponse {
        id: record.id,
        status: record.status,
        result,
        error: record.error,
    }))
}

/// 创建图片对话（LLaVA）
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Conversation created", body = LlavaConversationResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_llava_conversation(
    id: PathParam<i32>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<LlavaConversationResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;
    let redis = depot
        .obtain::<Arc<Mutex<redis::aio::ConnectionManager>>>()
        .map_err(|_| AppError::Internal("Redis connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: LlavaChatRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;
    let query = payload.query.trim();
    if query.is_empty() {
        return Err(AppError::BadRequest("Query cannot be empty".to_string()));
    }

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;
    if photo.user_id != user_id {
        return Err(AppError::Forbidden("Not your photo".to_string()));
    }

    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| config.processing.llava_default_model.clone());

    let now = Utc::now();
    let meta = build_llava_meta(&payload);
    let history = vec![ConversationMessage {
        role: "user".to_string(),
        content: query.to_string(),
        created_at: now.to_rfc3339(),
        meta,
    }];
    let history_value = serde_json::to_value(&history)
        .map_err(|e| AppError::Internal(format!("Failed to serialize history: {}", e)))?;

    let record = photo_conversations::ActiveModel {
        id: Default::default(),
        user_id: Set(user_id),
        photo_id: Set(photo.id),
        model: Set(model.clone()),
        status: Set("pending".to_string()),
        history: Set(Some(history_value.clone())),
        error: Set(None),
        created_at: Set(now.into()),
        updated_at: Set(now.into()),
    }
    .insert(db.as_ref())
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create conversation: {}", e)))?;

    let params = LlavaTaskParams::from_request(&payload);
    let image_path = resolve_photo_path(config.as_ref(), &photo.url);
    enqueue_llava_task(
        redis.clone(),
        &config.processing.llava_queue,
        record.id,
        record.photo_id,
        record.user_id,
        &image_path,
        &model,
        history_value,
        params,
    )
    .await?;

    Ok(Json(LlavaConversationResponse {
        id: record.id,
        status: record.status,
        history: record.history,
        error: record.error,
    }))
}

/// 继续图片对话（LLaVA）
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Conversation updated", body = LlavaConversationResponse),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn continue_llava_conversation(
    id: PathParam<i32>,
    conversation_id: PathParam<i64>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<LlavaConversationResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;
    let redis = depot
        .obtain::<Arc<Mutex<redis::aio::ConnectionManager>>>()
        .map_err(|_| AppError::Internal("Redis connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: LlavaChatRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;
    let query = payload.query.trim();
    if query.is_empty() {
        return Err(AppError::BadRequest("Query cannot be empty".to_string()));
    }

    let record = photo_conversations::Entity::find_by_id(*conversation_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;
    if record.user_id != user_id {
        return Err(AppError::Forbidden("Not your conversation".to_string()));
    }
    if record.photo_id != *id {
        return Err(AppError::BadRequest("Photo mismatch".to_string()));
    }
    if record.status == "pending" {
        return Err(AppError::BadRequest("Conversation pending".to_string()));
    }

    let mut history = parse_history(record.history.clone())?;
    let now = Utc::now();
    let meta = build_llava_meta(&payload);
    history.push(ConversationMessage {
        role: "user".to_string(),
        content: query.to_string(),
        created_at: now.to_rfc3339(),
        meta,
    });
    let history_value = serde_json::to_value(&history)
        .map_err(|e| AppError::Internal(format!("Failed to serialize history: {}", e)))?;

    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| record.model.clone());

    let mut active: photo_conversations::ActiveModel = record.into();
    active.status = Set("pending".to_string());
    active.model = Set(model.clone());
    active.history = Set(Some(history_value.clone()));
    active.error = Set(None);
    active.updated_at = Set(now.into());

    let saved = active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update conversation: {}", e)))?;

    let photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;
    let image_path = resolve_photo_path(config.as_ref(), &photo.url);
    let params = LlavaTaskParams::from_request(&payload);
    enqueue_llava_task(
        redis.clone(),
        &config.processing.llava_queue,
        saved.id,
        saved.photo_id,
        saved.user_id,
        &image_path,
        &model,
        history_value,
        params,
    )
    .await?;

    Ok(Json(LlavaConversationResponse {
        id: saved.id,
        status: saved.status,
        history: saved.history,
        error: saved.error,
    }))
}

/// 查询图片对话状态/结果
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Conversation", body = LlavaConversationResponse),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_llava_conversation(
    id: PathParam<i32>,
    conversation_id: PathParam<i64>,
    depot: &mut Depot,
) -> Result<Json<LlavaConversationResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let record = photo_conversations::Entity::find_by_id(*conversation_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;

    if record.user_id != user_id {
        return Err(AppError::Forbidden("Not your conversation".to_string()));
    }
    if record.photo_id != *id {
        return Err(AppError::BadRequest("Photo mismatch".to_string()));
    }

    Ok(Json(LlavaConversationResponse {
        id: record.id,
        status: record.status,
        history: record.history,
        error: record.error,
    }))
}

/// Runner 回调：写入文本 embedding 并触发向量搜索
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Updated and searched"),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found"),
        (status_code = 500, description = "Internal server error")
    )
)]
pub async fn update_text_query(
    id: PathParam<i64>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let token = req
        .headers()
        .get("x-internal-token")
        .and_then(|v| v.to_str().ok());
    if token != Some(config.processing.internal_token.as_str()) {
        return Err(AppError::Unauthorized("Invalid internal token".to_string()));
    }

    let payload: TextEmbedCallbackRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    let record = text_queries::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Query not found".to_string()))?;

    if payload.embedding.len() != config.processing.embedding_dim {
        return Err(AppError::BadRequest(format!(
            "Embedding dimension mismatch: expected {}, got {}",
            config.processing.embedding_dim,
            payload.embedding.len()
        )));
    }

    let limit = record.limit.unwrap_or(20) as i64;
    let model = payload
        .model
        .clone()
        .unwrap_or_else(|| record.model.clone());
    let filters = parse_text_search_filters(&record.query, chrono::Utc::now());

    let results =
        run_text_fusion_search(
            db.as_ref(),
            record.user_id,
            &model,
            &payload.embedding,
            limit,
            &filters,
        )
            .await?;

    let now = chrono::Utc::now();
    let mut active: text_queries::ActiveModel = record.into();
    active.status = Set("done".to_string());
    active.embedding = Set(Some(serde_json::to_value(&payload.embedding).map_err(
        |e| AppError::Internal(format!("Failed to serialize embedding: {}", e)),
    )?));
    active.result = Set(Some(serde_json::to_value(&results).map_err(|e| {
        AppError::Internal(format!("Failed to serialize result: {}", e))
    })?));
    active.error = Set(payload.error.clone());
    active.updated_at = Set(now.into());

    active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update text query: {}", e)))?;

    Ok(StatusCode::OK)
}

/// Runner 回调：写入 LLaVA 对话回复
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Conversation updated"),
        (status_code = 400, description = "Bad request"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 404, description = "Not found")
    )
)]
pub async fn update_llava_conversation(
    id: PathParam<i64>,
    req: &mut Request,
    depot: &mut Depot,
) -> Result<StatusCode> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let token = req
        .headers()
        .get("x-internal-token")
        .and_then(|v| v.to_str().ok());
    if token != Some(config.processing.internal_token.as_str()) {
        return Err(AppError::Unauthorized("Invalid internal token".to_string()));
    }

    let payload: LlavaCallbackRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;
    if payload.response.is_none() && payload.error.is_none() {
        return Err(AppError::BadRequest(
            "Either response or error must be provided".to_string(),
        ));
    }

    let record = photo_conversations::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Conversation not found".to_string()))?;

    let mut history = parse_history(record.history.clone())?;
    if let Some(response) = payload.response.clone() {
        history.push(ConversationMessage {
            role: "assistant".to_string(),
            content: response,
            created_at: Utc::now().to_rfc3339(),
            meta: None,
        });
    }
    let history_value = serde_json::to_value(&history)
        .map_err(|e| AppError::Internal(format!("Failed to serialize history: {}", e)))?;

    let mut active: photo_conversations::ActiveModel = record.into();
    active.status = Set(if payload.error.is_some() {
        "failed".to_string()
    } else {
        "done".to_string()
    });
    active.history = Set(Some(history_value));
    active.error = Set(payload.error.clone());
    active.updated_at = Set(Utc::now().into());

    active
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update conversation: {}", e)))?;

    Ok(StatusCode::OK)
}

/// Find similar photos using an existing photo's embedding
#[endpoint(
    tags("Photos"),
    responses(
        (status_code = 200, description = "Similar photos", body = Vec<SimilarPhotoResult>),
        (status_code = 400, description = "Embedding not found"),
        (status_code = 401, description = "Unauthorized"),
        (status_code = 500, description = "Internal server error")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn similar_photos(
    id: PathParam<i32>,
    depot: &mut Depot,
) -> Result<Json<Vec<SimilarPhotoResult>>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;
    let config = depot
        .obtain::<Arc<Config>>()
        .map_err(|_| AppError::Internal("Config not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let model = config.processing.default_embedding_model.clone();
    let modality = "image".to_string();

    // Verify ownership
    let base_photo = photos::Entity::find_by_id(*id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?;

    if base_photo.user_id != user_id {
        return Err(AppError::Forbidden(
            "Photo does not belong to user".to_string(),
        ));
    }

    // Ensure we have an embedding for the requested photo
    let exists_stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT 1 FROM photo_embeddings
        WHERE photo_id = $1 AND model = $2 AND modality = $3
        LIMIT 1
        "#,
        vec![
            Value::from(*id),
            Value::from(model.clone()),
            Value::from(modality.clone()),
        ],
    );

    let exists = db
        .query_one(exists_stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to check embedding: {}", e)))?;

    if exists.is_none() {
        return Err(AppError::BadRequest(
            "Embedding not found for this photo".to_string(),
        ));
    }

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        WITH base AS (
            SELECT embedding FROM photo_embeddings
        WHERE photo_id = $1 AND model = $2 AND modality = $3
        LIMIT 1
        )
        SELECT pe.photo_id, pe.embedding <=> (SELECT embedding FROM base) AS distance
        FROM photo_embeddings pe
        WHERE pe.model = $2
          AND pe.modality = $3
          AND pe.photo_id != $1
          AND EXISTS (
            SELECT 1 FROM photos p WHERE p.id = pe.photo_id AND p.user_id = $4
          )
        ORDER BY distance ASC
        LIMIT 20
        "#,
        vec![
            Value::from(*id),
            Value::from(model),
            Value::from(modality),
            Value::from(user_id),
        ],
    );

    let rows = db
        .query_all(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Similar search failed: {}", e)))?;

    let mut ordered: Vec<(i32, f64)> = Vec::new();
    for row in rows {
        let photo_id: i32 = row
            .try_get("", "photo_id")
            .map_err(|e| AppError::Internal(format!("Failed to decode photo_id: {}", e)))?;
        let distance: f64 = row
            .try_get("", "distance")
            .map_err(|e| AppError::Internal(format!("Failed to decode distance: {}", e)))?;
        ordered.push((photo_id, distance));
    }

    if ordered.is_empty() {
        return Ok(Json(vec![]));
    }

    let photo_ids: Vec<i32> = ordered.iter().map(|(id, _)| *id).collect();
    let models = photos::Entity::find()
        .filter(photos::Column::Id.is_in(photo_ids.clone()))
        .all(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let mut map: HashMap<i32, photos::Model> = models.into_iter().map(|m| (m.id, m)).collect();

    let mut results: Vec<SimilarPhotoResult> = Vec::new();
    for (pid, distance) in ordered {
        if let Some(photo) = map.remove(&pid) {
            results.push(SimilarPhotoResult {
                photo: PhotoResponse::from(photo),
                distance,
            });
        }
    }

    Ok(Json(results))
}

fn parse_datetime(value: &str) -> Result<chrono::NaiveDateTime> {
    chrono::DateTime::parse_from_rfc3339(value)
        .map(|dt| dt.naive_utc())
        .or_else(|_| chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S"))
        .map_err(|_| {
            AppError::BadRequest(
                "Invalid date_time_original format, use RFC3339 or YYYY-MM-DD HH:MM:SS".to_string(),
            )
        })
}

fn json_embedding_to_vec(value: &JsonValue) -> Option<Vec<f32>> {
    let arr = value.as_array()?;
    let mut vec = Vec::with_capacity(arr.len());
    for v in arr {
        if let Some(f) = v.as_f64() {
            vec.push(f as f32);
        } else if let Some(i) = v.as_i64() {
            vec.push(i as f32);
        } else {
            return None;
        }
    }
    Some(vec)
}

fn normalize(vec: &[f32]) -> Vec<f32> {
    let norm = vec.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm == 0.0 {
        return vec.to_vec();
    }
    vec.iter().map(|v| v / norm).collect()
}

fn cosine_distance(a: &[f32], b: &[f32]) -> f32 {
    let dot = a.iter().zip(b.iter()).map(|(x, y)| x * y).sum::<f32>();
    let norm_a = a.iter().map(|v| v * v).sum::<f32>().sqrt();
    let norm_b = b.iter().map(|v| v * v).sum::<f32>().sqrt();
    if norm_a == 0.0 || norm_b == 0.0 {
        return 1.0;
    }
    1.0 - dot / (norm_a * norm_b)
}

fn cluster_faces(faces: &[(i32, Vec<f32>)], threshold: f32) -> Vec<Vec<(i32, Vec<f32>)>> {
    let mut clusters: Vec<Vec<(i32, Vec<f32>)>> = Vec::new();

    for (pid, vec) in faces.iter() {
        let mut placed = false;
        for cluster in clusters.iter_mut() {
            // 使用第一张作为代表（向量已归一化）
            if let Some((_, center)) = cluster.first() {
                if cosine_distance(center, vec) <= threshold {
                    cluster.push((*pid, vec.clone()));
                    placed = true;
                    break;
                }
            }
        }

        if !placed {
            clusters.push(vec![(*pid, vec.clone())]);
        }
    }

    clusters
}

async fn generate_people_albums(
    db: &DatabaseConnection,
    user_id: i32,
    threshold: f32,
    min_faces: usize,
    model: &str,
    dry_run: bool,
) -> Result<Vec<PeopleAlbumResult>> {
    let records = face_embeddings::Entity::find()
        .join(JoinType::InnerJoin, face_embeddings::Relation::Photo.def())
        .filter(photos::Column::UserId.eq(user_id))
        .filter(face_embeddings::Column::Model.eq(model.to_string()))
        .all(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to load face embeddings: {}", e)))?;

    let mut faces: Vec<(i32, Vec<f32>)> = Vec::new();
    for rec in records {
        if let Some(vec) = json_embedding_to_vec(&rec.embedding) {
            faces.push((rec.photo_id, normalize(&vec)));
        }
    }

    if faces.is_empty() {
        return Ok(Vec::new());
    }

    let clusters = cluster_faces(&faces, threshold);

    let mut responses: Vec<PeopleAlbumResult> = Vec::new();
    for (idx, cluster) in clusters.iter().enumerate() {
        if cluster.len() < min_faces {
            continue;
        }
        let mut unique: Vec<i32> = cluster.iter().map(|(pid, _)| *pid).collect();
        unique.sort_unstable();
        unique.dedup();

        let name = format!("人物簇 #{}", idx + 1);
        let desc = format!(
            "自动聚类生成，faces={}，threshold={:.2}",
            cluster.len(),
            threshold
        );

        let album_id = if dry_run {
            None
        } else {
            Some(
                upsert_album_with_photos(db, user_id, &name, Some(desc), &unique).await?,
            )
        };

        responses.push(PeopleAlbumResult {
            album_id,
            name,
            face_count: cluster.len(),
            photo_ids: unique,
        });
    }

    Ok(responses)
}

/// 用于后台定时任务：发现有新的人脸写入后，为所有相关用户生成/更新人物相册
pub async fn run_auto_people_job(db: &DatabaseConnection, config: &Config) -> Result<()> {
    let model = config.processing.default_face_embedding_model.clone();
    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT DISTINCT p.user_id
        FROM face_embeddings fe
        JOIN photos p ON p.id = fe.photo_id
        WHERE fe.model = $1
        "#,
        vec![Value::from(model.clone())],
    );

    let rows = db
        .query_all(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list users for people albums: {}", e)))?;

    let mut user_ids: Vec<i32> = Vec::new();
    for row in rows {
        let uid: i32 = row
            .try_get("", "user_id")
            .map_err(|e| AppError::Internal(format!("Failed to decode user_id: {}", e)))?;
        user_ids.push(uid);
    }

    if user_ids.is_empty() {
        return Ok(());
    }

    for user_id in user_ids {
        // 忽略单个用户的失败，继续尝试其他用户
        if let Err(e) = generate_people_albums(
            db,
            user_id,
            config.processing.face_cluster_threshold,
            config.processing.face_cluster_min_faces,
            &model,
            false,
        )
        .await
        {
            tracing::error!("Auto people albums for user {} failed: {}", user_id, e);
        }
    }

    Ok(())
}

/// 调用外部文本 embedding 服务（如 ml/embed_service.py），返回 embedding
async fn fetch_text_embedding(query: &str, model: &str, url: &str) -> Result<Vec<f32>> {
    #[derive(Serialize)]
    struct Req<'a> {
        text: &'a str,
        model: &'a str,
    }

    #[derive(Deserialize)]
    struct Resp {
        embedding: Vec<f32>,
    }

    let client = reqwest::Client::new();
    let resp = client
        .post(url)
        .json(&Req { text: query, model })
        .send()
        .await
        .map_err(|e| AppError::Internal(format!("Failed to call text embed service: {}", e)))?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(AppError::Internal(format!(
            "Text embed service error: {} {}",
            status, body
        )));
    }

    let payload: Resp = resp
        .json()
        .await
        .map_err(|e| AppError::Internal(format!("Invalid embed service response: {}", e)))?;

    Ok(payload.embedding)
}

fn text_fusion_weights() -> (f64, f64, f64) {
    let image_w: f64 = env::var("TEXT_FUSION_IMAGE_WEIGHT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(1.0);
    let caption_w: f64 = env::var("TEXT_FUSION_CAPTION_WEIGHT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.0);
    let ocr_w: f64 = env::var("TEXT_FUSION_OCR_WEIGHT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0.2);
    (image_w.max(0.0), caption_w.max(0.0), ocr_w.max(0.0))
}

fn max_search_distance() -> Option<f64> {
    env::var("TEXT_SEARCH_MAX_DISTANCE")
        .ok()
        .and_then(|v| v.parse().ok())
}

async fn run_text_fusion_search(
    db: &DatabaseConnection,
    user_id: i32,
    model: &str,
    embedding: &[f32],
    limit: i64,
    filters: &TextSearchFilters,
) -> Result<Vec<SimilarPhotoResult>> {
    let (image_w, caption_w, ocr_w) = text_fusion_weights();
    let search_limit = (limit * 3).max(20);
    let mut agg: HashMap<i32, (PhotoResponse, f64, f64)> = HashMap::new();

    if image_w > 0.0 {
        let results = run_vector_search(
            db,
            user_id,
            model,
            "image",
            embedding,
            search_limit,
            filters.date_range,
            &filters.time_ranges,
        )
        .await?;
        for item in results {
            let entry = agg
                .entry(item.photo.id)
                .or_insert((item.photo.clone(), 0.0f64, 0.0f64));
            entry.1 += image_w * item.distance;
            entry.2 += image_w;
        }
    }

    if caption_w > 0.0 {
        let results = run_vector_search(
            db,
            user_id,
            model,
            "caption",
            embedding,
            search_limit,
            filters.date_range,
            &filters.time_ranges,
        )
        .await?;
        for item in results {
            let entry = agg
                .entry(item.photo.id)
                .or_insert((item.photo.clone(), 0.0f64, 0.0f64));
            entry.1 += caption_w * item.distance;
            entry.2 += caption_w;
        }
    }

    if ocr_w > 0.0 && !filters.ocr_terms.is_empty() {
        let results = run_ocr_match_search(
            db,
            user_id,
            &filters.ocr_terms,
            &filters.cleaned_query,
            filters.date_range,
            &filters.time_ranges,
            search_limit,
        )
        .await?;
        for item in results {
            let entry = agg
                .entry(item.photo.id)
                .or_insert((item.photo.clone(), 0.0f64, 0.0f64));
            entry.1 += ocr_w * item.distance;
            entry.2 += ocr_w;
        }
    }

    let mut fused: Vec<SimilarPhotoResult> = agg
        .into_iter()
        .filter_map(|(_, (photo, dist_sum, w_sum))| {
            if w_sum == 0.0 {
                None
            } else {
                Some(SimilarPhotoResult {
                    photo,
                    distance: dist_sum / w_sum,
                })
            }
        })
        .collect();

    fused.sort_by(|a, b| a.distance.partial_cmp(&b.distance).unwrap_or(std::cmp::Ordering::Equal));
    if let Some(max_dist) = max_search_distance() {
        fused.retain(|item| item.distance <= max_dist);
    }
    fused.truncate(limit as usize);
    Ok(fused)
}

async fn run_vector_search(
    db: &DatabaseConnection,
    user_id: i32,
    model: &str,
    modality: &str,
    embedding: &[f32],
    limit: i64,
    date_range: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
    time_ranges: &[(u8, u8)],
) -> Result<Vec<SimilarPhotoResult>> {
    let vector_literal = to_vector_literal(embedding);
    // CLIP 文本向量与图像向量同空间，数据库里只存 image 模态，因此文本查询也落在 image 上
    let target_modality = if modality == "text" {
        "image"
    } else {
        modality
    };

    let mut sql = String::from(
        r#"
        SELECT pe.photo_id, pe.embedding <=> $1::vector AS distance
        FROM photo_embeddings pe
        WHERE pe.model = $2
          AND pe.modality = $3
          AND EXISTS (
            SELECT 1 FROM photos p WHERE p.id = pe.photo_id AND p.user_id = $4
        "#,
    );
    let mut values = vec![
        Value::from(vector_literal),
        Value::from(model.to_string()),
        Value::from(target_modality.to_string()),
        Value::from(user_id),
    ];
    let mut bind_idx = 5;
    if let Some((start, end)) = date_range {
        sql.push_str(&format!(
            " AND COALESCE(p.date_time_original, p.created_at) >= ${} AND COALESCE(p.date_time_original, p.created_at) < ${}",
            bind_idx,
            bind_idx + 1
        ));
        values.push(Value::from(start));
        values.push(Value::from(end));
        bind_idx += 2;
    }
    if !time_ranges.is_empty() {
        let mut parts = Vec::new();
        for (start, end) in time_ranges {
            parts.push(format!(
                "(EXTRACT(HOUR FROM COALESCE(p.date_time_original, p.created_at)) >= ${} AND EXTRACT(HOUR FROM COALESCE(p.date_time_original, p.created_at)) < ${})",
                bind_idx,
                bind_idx + 1
            ));
            values.push(Value::from(*start as i32));
            values.push(Value::from(*end as i32));
            bind_idx += 2;
        }
        sql.push_str(" AND (");
        sql.push_str(&parts.join(" OR "));
        sql.push(')');
    }
    sql.push_str(
        r#"
          )
        ORDER BY distance ASC
        LIMIT "#,
    );
    sql.push_str(&format!("${}", bind_idx));
    values.push(Value::from(limit));

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        sql,
        values,
    );

    let rows = db
        .query_all(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Search query failed: {}", e)))?;

    let mut ordered: Vec<(i32, f64)> = Vec::new();
    for row in rows {
        let photo_id: i32 = row
            .try_get("", "photo_id")
            .map_err(|e| AppError::Internal(format!("Failed to decode photo_id: {}", e)))?;
        let distance: f64 = row
            .try_get("", "distance")
            .map_err(|e| AppError::Internal(format!("Failed to decode distance: {}", e)))?;
        ordered.push((photo_id, distance));
    }

    if ordered.is_empty() {
        return Ok(Vec::new());
    }

    let photo_ids: Vec<i32> = ordered.iter().map(|(id, _)| *id).collect();
    let models = photos::Entity::find()
        .filter(photos::Column::Id.is_in(photo_ids.clone()))
        .all(db)
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let mut map: HashMap<i32, photos::Model> = models.into_iter().map(|m| (m.id, m)).collect();

    let mut results: Vec<SimilarPhotoResult> = Vec::new();
    for (pid, distance) in ordered {
        if let Some(photo) = map.remove(&pid) {
            results.push(SimilarPhotoResult {
                photo: PhotoResponse::from(photo),
                distance,
            });
        }
    }

    Ok(results)
}

async fn run_ocr_match_search(
    db: &DatabaseConnection,
    user_id: i32,
    terms: &[String],
    full_query: &str,
    date_range: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
    time_ranges: &[(u8, u8)],
    limit: i64,
) -> Result<Vec<SimilarPhotoResult>> {
    if terms.is_empty() {
        return Ok(Vec::new());
    }

    let mut strong_matches: HashSet<i32> = HashSet::new();
    let full_query_trim = full_query.trim();
    let full_query_pattern = if full_query_trim.is_empty() {
        "__no_query__".to_string()
    } else {
        full_query_trim.to_string()
    };

    let mut sql = String::from(
        r#"
        SELECT
          p.id,
          CASE
            WHEN COALESCE(p.ml_result->>'ocr_text', p.ml_result->'ocr'->>'text') ILIKE $2 THEN 0
            WHEN p.description ILIKE $2 THEN 0
            ELSE 1
          END AS match_priority
        FROM photos p
        WHERE p.user_id = $1
        "#,
    );
    let mut values = vec![
        Value::from(user_id),
        Value::from(format!("%{}%", full_query_pattern)),
    ];
    let mut bind_idx = 3;

    let mut or_parts = Vec::new();
    for term in terms {
        or_parts.push(format!(
            "(p.description ILIKE ${} OR COALESCE(p.ml_result->>'ocr_text', p.ml_result->'ocr'->>'text') ILIKE ${})",
            bind_idx,
            bind_idx
        ));
        values.push(Value::from(format!("%{}%", term)));
        bind_idx += 1;
    }

    if !or_parts.is_empty() {
        sql.push_str(" AND (");
        sql.push_str(&or_parts.join(" OR "));
        sql.push(')');
    }

    if let Some((start, end)) = date_range {
        sql.push_str(&format!(
            " AND COALESCE(p.date_time_original, p.created_at) >= ${} AND COALESCE(p.date_time_original, p.created_at) < ${}",
            bind_idx,
            bind_idx + 1
        ));
        values.push(Value::from(start));
        values.push(Value::from(end));
        bind_idx += 2;
    }
    if !time_ranges.is_empty() {
        let mut parts = Vec::new();
        for (start, end) in time_ranges {
            parts.push(format!(
                "(EXTRACT(HOUR FROM COALESCE(p.date_time_original, p.created_at)) >= ${} AND EXTRACT(HOUR FROM COALESCE(p.date_time_original, p.created_at)) < ${})",
                bind_idx,
                bind_idx + 1
            ));
            values.push(Value::from(*start as i32));
            values.push(Value::from(*end as i32));
            bind_idx += 2;
        }
        sql.push_str(" AND (");
        sql.push_str(&parts.join(" OR "));
        sql.push(')');
    }

    sql.push_str(" ORDER BY match_priority ASC, COALESCE(p.date_time_original, p.created_at) DESC NULLS LAST LIMIT ");
    sql.push_str(&format!("${}", bind_idx));
    values.push(Value::from(limit));

    let stmt = Statement::from_sql_and_values(DbBackend::Postgres, sql, values);
    let rows = db
        .query_all(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("OCR search query failed: {}", e)))?;

    let mut ordered: Vec<i32> = Vec::new();
    for row in rows {
        let photo_id: i32 = row
            .try_get("", "id")
            .map_err(|e| AppError::Internal(format!("Failed to decode photo_id: {}", e)))?;
        let priority: i32 = row
            .try_get("", "match_priority")
            .unwrap_or(1);
        ordered.push(photo_id);
        if priority == 0 {
            strong_matches.insert(photo_id);
        }
    }

    if ordered.is_empty() {
        return Ok(Vec::new());
    }

    let models = photos::Entity::find()
        .filter(photos::Column::Id.is_in(ordered.clone()))
        .all(db)
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    let mut map: HashMap<i32, photos::Model> = models.into_iter().map(|m| (m.id, m)).collect();
    let mut results = Vec::new();
    for pid in ordered {
        if let Some(photo) = map.remove(&pid) {
            results.push(SimilarPhotoResult {
                photo: PhotoResponse::from(photo),
                distance: if strong_matches.contains(&pid) { 0.05 } else { 0.5 },
            });
        }
    }

    Ok(results)
}

fn parse_text_search_filters(
    query: &str,
    now: chrono::DateTime<chrono::Utc>,
) -> TextSearchFilters {
    let mut cleaned = query.to_string();
    let mut date_range: Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)> = None;
    let mut time_ranges: Vec<(u8, u8)> = Vec::new();

    let now_naive = now.naive_utc();
    let today = now.date_naive();
    let today_start = today.and_hms_opt(0, 0, 0).unwrap_or(now_naive);
    let week_start = start_of_week(today).and_hms_opt(0, 0, 0).unwrap_or(today_start);
    let month_start = start_of_month(today).and_hms_opt(0, 0, 0).unwrap_or(today_start);
    let last_week_start = week_start - chrono::Duration::days(7);
    let last_week_end = week_start;
    let last_month_start = start_of_prev_month(today).and_hms_opt(0, 0, 0).unwrap_or(today_start);
    let last_year_start = chrono::NaiveDate::from_ymd_opt(today.year() - 1, 1, 1)
        .unwrap_or(today)
        .and_hms_opt(0, 0, 0)
        .unwrap_or(today_start);
    let last_year_end = chrono::NaiveDate::from_ymd_opt(today.year(), 1, 1)
        .unwrap_or(today)
        .and_hms_opt(0, 0, 0)
        .unwrap_or(today_start);
    let this_year_start = chrono::NaiveDate::from_ymd_opt(today.year(), 1, 1)
        .unwrap_or(today)
        .and_hms_opt(0, 0, 0)
        .unwrap_or(today_start);

    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(7), now_naive), "最近一周");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(7), now_naive), "最近7天");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(30), now_naive), "最近30天");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(30), now_naive), "最近一月");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(30), now_naive), "最近1月");
    apply_date_range(&mut cleaned, &mut date_range, (last_week_start, last_week_end), "上周");
    apply_date_range(&mut cleaned, &mut date_range, (week_start, now_naive), "本周");
    apply_date_range(&mut cleaned, &mut date_range, (last_week_start, last_week_end), "上星期");
    apply_date_range(&mut cleaned, &mut date_range, (week_start, now_naive), "这周");
    apply_date_range(&mut cleaned, &mut date_range, (month_start, now_naive), "本月");
    apply_date_range(&mut cleaned, &mut date_range, (last_month_start, month_start), "上个月");
    apply_date_range(&mut cleaned, &mut date_range, (today_start - chrono::Duration::days(1), today_start), "昨天");
    apply_date_range(&mut cleaned, &mut date_range, (today_start, today_start + chrono::Duration::days(1)), "今天");
    apply_date_range(&mut cleaned, &mut date_range, (today_start, today_start + chrono::Duration::days(1)), "今日");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(182), now_naive), "半年前");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(182), now_naive), "半年内");
    apply_date_range(&mut cleaned, &mut date_range, (now_naive - chrono::Duration::days(182), now_naive), "最近半年");
    apply_date_range(&mut cleaned, &mut date_range, (last_year_start, last_year_end), "去年");
    apply_date_range(&mut cleaned, &mut date_range, (this_year_start, now_naive), "今年");

    let lower = cleaned.to_lowercase();
    if date_range.is_none() {
        if lower.contains("last week") {
            cleaned = replace_case_insensitive(&cleaned, "last week");
            date_range = Some((last_week_start, last_week_end));
        } else if lower.contains("this week") {
            cleaned = replace_case_insensitive(&cleaned, "this week");
            date_range = Some((week_start, now_naive));
        } else if lower.contains("last month") {
            cleaned = replace_case_insensitive(&cleaned, "last month");
            date_range = Some((last_month_start, month_start));
        } else if lower.contains("last year") {
            cleaned = replace_case_insensitive(&cleaned, "last year");
            date_range = Some((last_year_start, last_year_end));
        } else if lower.contains("this year") {
            cleaned = replace_case_insensitive(&cleaned, "this year");
            date_range = Some((this_year_start, now_naive));
        } else if lower.contains("this month") {
            cleaned = replace_case_insensitive(&cleaned, "this month");
            date_range = Some((month_start, now_naive));
        } else if lower.contains("today") {
            cleaned = replace_case_insensitive(&cleaned, "today");
            date_range = Some((today_start, today_start + chrono::Duration::days(1)));
        } else if lower.contains("yesterday") {
            cleaned = replace_case_insensitive(&cleaned, "yesterday");
            date_range = Some((today_start - chrono::Duration::days(1), today_start));
        }
    }

    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(6, 12)], "上午");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(11, 13)], "中午");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(12, 18)], "下午");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(17, 20)], "傍晚");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(18, 22)], "晚上");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(22, 24), (0, 6)], "夜间");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(22, 24), (0, 6)], "夜里");
    apply_time_ranges(&mut cleaned, &mut time_ranges, &[(0, 6)], "凌晨");

    cleaned = cleaned.replace("拍摄的", " ");
    cleaned = cleaned.replace("拍摄", " ");

    let cleaned = cleaned
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join(" ");

    let mut ocr_terms = Vec::new();
    let cleaned_trim = cleaned.trim();
    if !cleaned_trim.is_empty() {
        ocr_terms.push(cleaned_trim.to_string());
    }
    ocr_terms.extend(
        cleaned
            .split_whitespace()
            .map(|s| s.trim_matches(|c: char| c.is_ascii_punctuation()))
            .filter(|s| !s.is_empty())
            .map(|s| s.to_string()),
    );

    TextSearchFilters {
        date_range,
        ocr_terms,
        cleaned_query: cleaned,
        time_ranges,
    }
}

fn replace_case_insensitive(source: &str, target: &str) -> String {
    let lower = source.to_lowercase();
    let target_lower = target.to_lowercase();
    if let Some(pos) = lower.find(&target_lower) {
        let mut out = String::new();
        out.push_str(&source[..pos]);
        out.push(' ');
        out.push_str(&source[pos + target.len()..]);
        out
    } else {
        source.to_string()
    }
}

fn consume_keyword(target: &mut String, needle: &str) -> bool {
    if target.contains(needle) {
        *target = target.replace(needle, " ");
        return true;
    }
    false
}

fn apply_date_range(
    cleaned: &mut String,
    date_range: &mut Option<(chrono::NaiveDateTime, chrono::NaiveDateTime)>,
    range: (chrono::NaiveDateTime, chrono::NaiveDateTime),
    needle: &str,
) {
    if consume_keyword(cleaned, needle) {
        *date_range = Some(range);
    }
}

fn apply_time_ranges(
    cleaned: &mut String,
    time_ranges: &mut Vec<(u8, u8)>,
    ranges: &[(u8, u8)],
    needle: &str,
) {
    if consume_keyword(cleaned, needle) {
        for r in ranges {
            if !time_ranges.contains(r) {
                time_ranges.push(*r);
            }
        }
    }
}


fn start_of_week(date: chrono::NaiveDate) -> chrono::NaiveDate {
    let weekday = date.weekday().num_days_from_monday() as i64;
    date - chrono::Duration::days(weekday)
}

fn start_of_month(date: chrono::NaiveDate) -> chrono::NaiveDate {
    chrono::NaiveDate::from_ymd_opt(date.year(), date.month(), 1).unwrap_or(date)
}

fn start_of_prev_month(date: chrono::NaiveDate) -> chrono::NaiveDate {
    let (year, month) = if date.month() == 1 {
        (date.year() - 1, 12)
    } else {
        (date.year(), date.month() - 1)
    };
    chrono::NaiveDate::from_ymd_opt(year, month, 1).unwrap_or(date)
}

fn merge_ml_result(existing: Option<JsonValue>, incoming: JsonValue) -> JsonValue {
    match existing {
        Some(mut base) => {
            merge_json_value(&mut base, incoming);
            base
        }
        None => incoming,
    }
}

fn merge_json_value(target: &mut JsonValue, incoming: JsonValue) {
    match (target, incoming) {
        (JsonValue::Object(target_map), JsonValue::Object(incoming_map)) => {
            for (key, value) in incoming_map {
                match target_map.get_mut(&key) {
                    Some(existing) => merge_json_value(existing, value),
                    None => {
                        target_map.insert(key, value);
                    }
                }
            }
        }
        (target_slot, incoming_value) => {
            *target_slot = incoming_value;
        }
    }
}

#[derive(Debug, Serialize)]
struct LlavaTaskParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    temperature: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    top_p: Option<f32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    num_beams: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_new_tokens: Option<u32>,
}

impl LlavaTaskParams {
    fn from_request(req: &LlavaChatRequest) -> Self {
        Self {
            temperature: req.temperature,
            top_p: req.top_p,
            num_beams: req.num_beams,
            max_new_tokens: req.max_new_tokens,
        }
    }

    fn is_empty(&self) -> bool {
        self.temperature.is_none()
            && self.top_p.is_none()
            && self.num_beams.is_none()
            && self.max_new_tokens.is_none()
    }
}

fn build_llava_meta(req: &LlavaChatRequest) -> Option<JsonValue> {
    let mut map = serde_json::Map::new();
    if let Some(value) = req.temperature {
        map.insert("temperature".to_string(), JsonValue::from(value));
    }
    if let Some(value) = req.top_p {
        map.insert("top_p".to_string(), JsonValue::from(value));
    }
    if let Some(value) = req.num_beams {
        map.insert("num_beams".to_string(), JsonValue::from(value));
    }
    if let Some(value) = req.max_new_tokens {
        map.insert("max_new_tokens".to_string(), JsonValue::from(value));
    }
    if map.is_empty() {
        None
    } else {
        Some(JsonValue::Object(map))
    }
}

fn parse_history(history: Option<JsonValue>) -> Result<Vec<ConversationMessage>> {
    match history {
        Some(value) => serde_json::from_value(value)
            .map_err(|e| AppError::Internal(format!("Invalid conversation history: {}", e))),
        None => Ok(Vec::new()),
    }
}

fn resolve_photo_path(config: &Config, url: &str) -> String {
    let filename = PathBuf::from(url)
        .file_name()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| url.trim_start_matches('/').to_string());
    PathBuf::from(&config.storage.upload_dir)
        .join(filename)
        .to_string_lossy()
        .to_string()
}

async fn enqueue_llava_task(
    redis: Arc<Mutex<redis::aio::ConnectionManager>>,
    queue: &str,
    conversation_id: i64,
    photo_id: i32,
    user_id: i32,
    path: &str,
    model: &str,
    history: JsonValue,
    params: LlavaTaskParams,
) -> Result<()> {
    #[derive(Serialize)]
    struct Payload<'a> {
        conversation_id: i64,
        photo_id: i32,
        user_id: i32,
        path: &'a str,
        model: &'a str,
        history: JsonValue,
        #[serde(skip_serializing_if = "Option::is_none")]
        params: Option<LlavaTaskParams>,
    }

    let payload = Payload {
        conversation_id,
        photo_id,
        user_id,
        path,
        model,
        history,
        params: if params.is_empty() { None } else { Some(params) },
    };

    let serialized = serde_json::to_string(&payload)
        .map_err(|e| AppError::Internal(format!("Failed to serialize llava task: {}", e)))?;

    let mut conn = redis.lock().await;
    let _: i64 = conn
        .lpush(queue, serialized)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to push llava task: {}", e)))?;
    Ok(())
}

async fn enqueue_text_task(
    redis: Arc<Mutex<redis::aio::ConnectionManager>>,
    queue: &str,
    request_id: i64,
    query: &str,
    model: &str,
    limit: i64,
) -> Result<()> {
    #[derive(Serialize)]
    struct Payload<'a> {
        r#type: &'a str,
        request_id: i64,
        query: &'a str,
        model: &'a str,
        limit: i64,
    }

    let payload = Payload {
        r#type: "text_embed",
        request_id,
        query,
        model,
        limit,
    };

    let serialized = serde_json::to_string(&payload)
        .map_err(|e| AppError::Internal(format!("Failed to serialize text task: {}", e)))?;

    let mut conn = redis.lock().await;
    let _: i64 = conn
        .lpush(queue, serialized)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to push text task: {}", e)))?;
    Ok(())
}

async fn upsert_album_with_photos(
    db: &DatabaseConnection,
    user_id: i32,
    name: &str,
    description: Option<String>,
    photo_ids: &[i32],
) -> Result<i32> {
    if let Some(mut existing) = albums::Entity::find()
        .filter(albums::Column::UserId.eq(user_id))
        .filter(albums::Column::Name.eq(name))
        .one(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to load album: {}", e)))?
    {
        let album_id = existing.id;
        // 更新描述以反映最新的聚类参数
        existing.description = description;
        let mut active: albums::ActiveModel = existing.into();
        active.updated_at = Set(chrono::Utc::now().naive_utc());
        active
            .update(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update album: {}", e)))?;

        let existing_ids: HashSet<i32> = album_photos::Entity::find()
            .filter(album_photos::Column::AlbumId.eq(album_id))
            .all(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to load album photos: {}", e)))?
            .into_iter()
            .map(|ap| ap.photo_id)
            .collect();

        for pid in photo_ids {
            if existing_ids.contains(pid) {
                continue;
            }
            let rel = album_photos::ActiveModel {
                album_id: Set(album_id),
                photo_id: Set(*pid),
                created_at: Set(chrono::Utc::now().naive_utc()),
                ..Default::default()
            };
            rel.insert(db)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to attach photo to album: {}", e)))?;
        }

        return Ok(album_id);
    }

    Ok(
        create_album_with_photos(db, user_id, name, description, photo_ids)
            .await?
            .id,
    )
}

async fn create_album_with_photos(
    db: &DatabaseConnection,
    user_id: i32,
    name: &str,
    description: Option<String>,
    photo_ids: &[i32],
) -> Result<albums::Model> {
    // 确保照片属于用户
    let owned_photos = photos::Entity::find()
        .filter(photos::Column::Id.is_in(photo_ids.to_vec()))
        .filter(photos::Column::UserId.eq(user_id))
        .all(db)
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?;

    if owned_photos.len() != photo_ids.len() {
        return Err(AppError::Forbidden(
            "包含不属于你的照片或照片不存在".to_string(),
        ));
    }

    let now = chrono::Utc::now().naive_utc();
    let album = albums::ActiveModel {
        user_id: Set(user_id),
        name: Set(name.to_string()),
        description: Set(description),
        cover_photo_id: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    }
    .insert(db)
    .await
    .map_err(|e| AppError::Internal(format!("Failed to create album: {}", e)))?;

    for pid in photo_ids {
        let active = album_photos::ActiveModel {
            album_id: Set(album.id),
            photo_id: Set(*pid),
            created_at: Set(chrono::Utc::now().naive_utc()),
            ..Default::default()
        };

        active
            .insert(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to attach photo to album: {}", e)))?;
    }

    Ok(album)
}

fn to_vector_literal(values: &[f32]) -> String {
    let parts: Vec<String> = values.iter().map(|v| format!("{:.6}", v)).collect();
    format!("[{}]", parts.join(","))
}

async fn enqueue_processing_task(
    redis: Arc<Mutex<redis::aio::ConnectionManager>>,
    queue: &str,
    task: ProcessingTask,
) -> Result<()> {
    let payload = serde_json::to_string(&task)
        .map_err(|e| AppError::Internal(format!("Failed to serialize task: {}", e)))?;
    let mut conn = redis.lock().await;
    let _: i64 = conn
        .lpush(queue, payload)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to push task to Redis: {}", e)))?;
    Ok(())
}

/// Background task to generate thumbnail and update database
async fn generate_thumbnail_background(
    photo_id: i32,
    original_path: PathBuf,
    filename: String,
    db: Arc<DatabaseConnection>,
    config: Arc<crate::config::Config>,
) -> Result<()> {
    // Generate thumbnail in blocking thread pool (CPU-intensive operation)
    let thumbnail_path = PathBuf::from(&config.storage.thumbnail_dir).join(&filename);
    let width = config.storage.thumbnail_width;
    let height = config.storage.thumbnail_height;

    tokio::task::spawn_blocking(move || {
        img_utils::generate_thumbnail(&original_path, &thumbnail_path, width, height)
    })
    .await
    .map_err(|e| AppError::Internal(format!("Failed to join task: {}", e)))??;

    // Update database with thumbnail URL
    let mut photo: photos::ActiveModel = photos::Entity::find_by_id(photo_id)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?
        .into();

    photo.thumbnail_url = Set(Some(format!("/uploads/thumbnails/{}", filename)));
    let existing_status = photo.ml_status.take().flatten();
    photo.ml_status = Set(Some(ml_status::merge_ml_status(
        existing_status,
        &[("thumbnail", "done")],
    )));
    photo.updated_at = Set(chrono::Utc::now().naive_utc());

    photo
        .update(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update photo with thumbnail: {}", e)))?;

    Ok(())
}
