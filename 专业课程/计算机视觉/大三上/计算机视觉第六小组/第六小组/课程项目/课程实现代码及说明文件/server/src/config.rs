use crate::error::{AppError, Result};
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
    pub processing: ProcessingConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StorageConfig {
    /// 原图存储路径
    pub upload_dir: String,
    /// 缩略图存储路径
    pub thumbnail_dir: String,
    /// 缩略图宽度
    pub thumbnail_width: u32,
    /// 缩略图高度
    pub thumbnail_height: u32,
    /// 允许的最大文件大小（字节）
    pub max_file_size: usize,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ProcessingConfig {
    /// Shared secret used by the ML runner when calling internal callbacks
    pub internal_token: String,
    /// Embedding dimension (e.g. 768 for CLIP ViT-L/14, 512 for ViT-B/32)
    pub embedding_dim: usize,
    /// Default embedding model name to record in the database
    pub default_embedding_model: String,
    /// Face embedding dimension (e.g. 512 for ArcFace)
    pub face_embedding_dim: usize,
    /// Default face embedding model label
    pub default_face_embedding_model: String,
    /// Threshold for clustering faces (cosine distance, lower is stricter)
    pub face_cluster_threshold: f32,
    /// Minimum faces per cluster to create an album
    pub face_cluster_min_faces: usize,
    /// Optional HTTP endpoint that returns text embeddings; if None, search_by_text requires embedding
    pub text_embed_url: Option<String>,
    /// Redis queue name for async text embedding tasks
    pub text_embed_queue: String,
    /// Redis queue name for LLaVA conversation tasks
    pub llava_queue: String,
    /// Default LLaVA model name used by the API
    pub llava_default_model: String,
    /// Interval seconds for ML planner job
    pub planner_interval_secs: u64,
    /// Batch size for ML planner job
    pub planner_batch_size: u32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub queue_name: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        // Load .env file
        dotenvy::dotenv().ok();

        let server = ServerConfig {
            host: std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string()),
            port: std::env::var("SERVER_PORT")
                .unwrap_or_else(|_| "5800".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid SERVER_PORT".to_string()))?,
        };

        let database = DatabaseConfig {
            url: std::env::var("DATABASE_URL")
                .map_err(|_| AppError::Config("DATABASE_URL must be set".to_string()))?,
            max_connections: std::env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "10".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid DB_MAX_CONNECTIONS".to_string()))?,
            min_connections: std::env::var("DB_MIN_CONNECTIONS")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid DB_MIN_CONNECTIONS".to_string()))?,
        };

        let storage = StorageConfig {
            upload_dir: std::env::var("UPLOAD_DIR")
                .unwrap_or_else(|_| "./uploads/original".to_string()),
            thumbnail_dir: std::env::var("THUMBNAIL_DIR")
                .unwrap_or_else(|_| "./uploads/thumbnails".to_string()),
            thumbnail_width: std::env::var("THUMBNAIL_WIDTH")
                .unwrap_or_else(|_| "400".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid THUMBNAIL_WIDTH".to_string()))?,
            thumbnail_height: std::env::var("THUMBNAIL_HEIGHT")
                .unwrap_or_else(|_| "400".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid THUMBNAIL_HEIGHT".to_string()))?,
            max_file_size: std::env::var("MAX_FILE_SIZE")
                .unwrap_or_else(|_| "104857600".to_string()) // 100MB
                .parse()
                .map_err(|_| AppError::Config("Invalid MAX_FILE_SIZE".to_string()))?,
        };

        let processing = ProcessingConfig {
            internal_token: std::env::var("PROCESSING_INTERNAL_TOKEN")
                .unwrap_or_else(|_| "change-me".to_string()),
            embedding_dim: std::env::var("PROCESSING_EMBEDDING_DIM")
                .unwrap_or_else(|_| "768".to_string())
                .parse()
                .map_err(|_| AppError::Config("Invalid PROCESSING_EMBEDDING_DIM".to_string()))?,
            default_embedding_model: std::env::var("PROCESSING_DEFAULT_MODEL")
                .unwrap_or_else(|_| "chinese-clip-vit-large-patch14".to_string()),
            face_embedding_dim: std::env::var("PROCESSING_FACE_EMBEDDING_DIM")
                .unwrap_or_else(|_| "512".to_string())
                .parse()
                .map_err(|_| {
                    AppError::Config("Invalid PROCESSING_FACE_EMBEDDING_DIM".to_string())
                })?,
            default_face_embedding_model: std::env::var("PROCESSING_DEFAULT_FACE_MODEL")
                .unwrap_or_else(|_| "arcface-buffalo_l".to_string()),
            face_cluster_threshold: std::env::var("PROCESSING_FACE_CLUSTER_THRESHOLD")
                .unwrap_or_else(|_| "0.42".to_string())
                .parse()
                .map_err(|_| {
                    AppError::Config("Invalid PROCESSING_FACE_CLUSTER_THRESHOLD".to_string())
                })?,
            face_cluster_min_faces: std::env::var("PROCESSING_FACE_CLUSTER_MIN_FACES")
                .unwrap_or_else(|_| "2".to_string())
                .parse()
                .map_err(|_| {
                    AppError::Config("Invalid PROCESSING_FACE_CLUSTER_MIN_FACES".to_string())
                })?,
            text_embed_url: std::env::var("PROCESSING_TEXT_EMBED_URL").ok(),
            text_embed_queue: std::env::var("TEXT_EMBED_QUEUE_NAME")
                .unwrap_or_else(|_| "text_tasks".to_string()),
            llava_queue: std::env::var("LLAVA_QUEUE_NAME")
                .unwrap_or_else(|_| "llava_tasks".to_string()),
            llava_default_model: std::env::var("LLAVA_MODEL_NAME")
                .unwrap_or_else(|_| "liuhaotian/llava-v1.5-7b".to_string()),
            planner_interval_secs: std::env::var("PROCESSING_PLANNER_INTERVAL_SECS")
                .unwrap_or_else(|_| "120".to_string())
                .parse()
                .map_err(|_| {
                    AppError::Config("Invalid PROCESSING_PLANNER_INTERVAL_SECS".to_string())
                })?,
            planner_batch_size: std::env::var("PROCESSING_PLANNER_BATCH_SIZE")
                .unwrap_or_else(|_| "50".to_string())
                .parse()
                .map_err(|_| {
                    AppError::Config("Invalid PROCESSING_PLANNER_BATCH_SIZE".to_string())
                })?,
        };

        let redis = RedisConfig {
            url: std::env::var("REDIS_URL").unwrap_or_else(|_| "redis://127.0.0.1/".to_string()),
            queue_name: std::env::var("REDIS_QUEUE_NAME")
                .unwrap_or_else(|_| "photo_tasks".to_string()),
        };

        Ok(Config {
            server,
            database,
            storage,
            processing,
            redis,
        })
    }
}
