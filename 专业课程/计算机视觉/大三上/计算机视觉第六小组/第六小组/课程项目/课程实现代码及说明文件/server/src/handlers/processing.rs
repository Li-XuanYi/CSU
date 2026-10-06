use redis::AsyncCommands;
use salvo::oapi::ToSchema;
use salvo::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbBackend, EntityTrait,
    FromQueryResult, QueryFilter, QuerySelect, Set, Statement, Value,
};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::config::Config;
use crate::error::{AppError, Result};
use crate::models::{photos, settings};
use crate::utils::ml_status;

const CONTROL_KEY: &str = "ml_control";
const TASKS: [&str; 4] = ["thumbnail", "clip", "face", "ocr"];
const RUNNER_TASKS: [&str; 3] = ["clip", "face", "ocr"];

#[derive(Debug, Serialize, ToSchema)]
pub struct TaskProgress {
    pub task: String,
    pub total: i64,
    pub done: i64,
    pub pending: i64,
    pub state: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProcessingProgressResponse {
    pub tasks: Vec<TaskProgress>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ProcessingControlRequest {
    pub task: String,
    pub action: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ProcessingControlResponse {
    pub task: String,
    pub action: String,
    pub state: String,
}

fn default_controls() -> HashMap<String, String> {
    let mut map = HashMap::new();
    for task in TASKS {
        map.insert(task.to_string(), "running".to_string());
    }
    map
}

fn normalize_controls(mut controls: HashMap<String, String>) -> HashMap<String, String> {
    for task in TASKS {
        controls
            .entry(task.to_string())
            .or_insert_with(|| "running".to_string());
    }
    controls
}

async fn load_controls(db: &DatabaseConnection, user_id: i32) -> Result<HashMap<String, String>> {
    let record = settings::Entity::find()
        .filter(settings::Column::UserId.eq(user_id))
        .filter(settings::Column::Key.eq(CONTROL_KEY))
        .one(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to load controls: {}", e)))?;

    if let Some(record) = record {
        if let Ok(map) = serde_json::from_str::<HashMap<String, String>>(&record.value) {
            return Ok(normalize_controls(map));
        }
    }

    Ok(default_controls())
}

async fn save_controls(
    db: &DatabaseConnection,
    user_id: i32,
    controls: &HashMap<String, String>,
) -> Result<()> {
    let value = serde_json::to_string(controls)
        .map_err(|e| AppError::Internal(format!("Failed to serialize controls: {}", e)))?;

    let existing = settings::Entity::find()
        .filter(settings::Column::UserId.eq(user_id))
        .filter(settings::Column::Key.eq(CONTROL_KEY))
        .one(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to load controls: {}", e)))?;

    let now = chrono::Utc::now().naive_utc();
    if let Some(record) = existing {
        let mut active: settings::ActiveModel = record.into();
        active.value = Set(value);
        active.updated_at = Set(now);
        active
            .update(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to update controls: {}", e)))?;
    } else {
        let active = settings::ActiveModel {
            id: Set(0),
            user_id: Set(user_id),
            key: Set(CONTROL_KEY.to_string()),
            value: Set(value),
            created_at: Set(now),
            updated_at: Set(now),
        };
        active
            .insert(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to insert controls: {}", e)))?;
    }
    Ok(())
}

#[derive(Debug, sea_orm::FromQueryResult)]
struct ProgressRow {
    clip_total: i64,
    clip_done: i64,
    face_total: i64,
    face_done: i64,
    ocr_total: i64,
    ocr_done: i64,
    thumbnail_total: i64,
    thumbnail_done: i64,
}

fn progress_item(task: &str, total: i64, done: i64, state: &str) -> TaskProgress {
    TaskProgress {
        task: task.to_string(),
        total,
        done,
        pending: total - done,
        state: state.to_string(),
    }
}

#[endpoint(
    tags("Processing"),
    responses((status_code = 200, description = "Progress", body = ProcessingProgressResponse)),
    security(("bearer_auth" = []))
)]
pub async fn get_processing_progress(
    depot: &mut Depot,
) -> Result<Json<ProcessingProgressResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let controls = load_controls(db.as_ref(), user_id).await?;

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        SELECT
          COUNT(*) FILTER (WHERE ml_status ? 'clip') AS clip_total,
          COUNT(*) FILTER (WHERE ml_status->>'clip' = 'done') AS clip_done,
          COUNT(*) FILTER (WHERE ml_status ? 'face') AS face_total,
          COUNT(*) FILTER (WHERE ml_status->>'face' = 'done') AS face_done,
          COUNT(*) FILTER (WHERE ml_status ? 'ocr') AS ocr_total,
          COUNT(*) FILTER (WHERE ml_status->>'ocr' = 'done') AS ocr_done,
          COUNT(*) FILTER (WHERE ml_status ? 'thumbnail') AS thumbnail_total,
          COUNT(*) FILTER (WHERE ml_status->>'thumbnail' = 'done') AS thumbnail_done
        FROM photos
        WHERE user_id = $1
        "#,
        vec![Value::from(user_id)],
    );

    let row = ProgressRow::find_by_statement(stmt)
        .one(db.as_ref())
        .await
        .map_err(|e| AppError::Internal(format!("Failed to load progress: {}", e)))?
        .unwrap_or(ProgressRow {
            clip_total: 0,
            clip_done: 0,
            face_total: 0,
            face_done: 0,
            ocr_total: 0,
            ocr_done: 0,
            thumbnail_total: 0,
            thumbnail_done: 0,
        });

    let tasks = vec![
        progress_item(
            "thumbnail",
            row.thumbnail_total,
            row.thumbnail_done,
            controls
                .get("thumbnail")
                .map(|s| s.as_str())
                .unwrap_or("running"),
        ),
        progress_item(
            "clip",
            row.clip_total,
            row.clip_done,
            controls.get("clip").map(|s| s.as_str()).unwrap_or("running"),
        ),
        progress_item(
            "face",
            row.face_total,
            row.face_done,
            controls.get("face").map(|s| s.as_str()).unwrap_or("running"),
        ),
        progress_item(
            "ocr",
            row.ocr_total,
            row.ocr_done,
            controls.get("ocr").map(|s| s.as_str()).unwrap_or("running"),
        ),
    ];

    Ok(Json(ProcessingProgressResponse { tasks }))
}

#[endpoint(
    tags("Processing"),
    responses((status_code = 200, description = "Control updated", body = ProcessingControlResponse)),
    security(("bearer_auth" = []))
)]
pub async fn update_processing_control(
    req: &mut Request,
    depot: &mut Depot,
) -> Result<Json<ProcessingControlResponse>> {
    let db = depot
        .obtain::<Arc<DatabaseConnection>>()
        .map_err(|_| AppError::Internal("Database connection not found in depot".to_string()))?;

    let user_id: i32 = depot
        .get::<String>("user_id")
        .ok()
        .and_then(|id| id.parse().ok())
        .ok_or_else(|| AppError::Unauthorized("User ID not found".to_string()))?;

    let payload: ProcessingControlRequest = req
        .parse_json()
        .await
        .map_err(|e| AppError::BadRequest(format!("Invalid JSON payload: {}", e)))?;

    let task = payload.task.to_lowercase();
    let action = payload.action.to_lowercase();

    if !TASKS.contains(&task.as_str()) {
        return Err(AppError::BadRequest("Invalid task".to_string()));
    }

    let mut controls = load_controls(db.as_ref(), user_id).await?;

    match action.as_str() {
        "pause" => {
            controls.insert(task.clone(), "paused".to_string());
            save_controls(db.as_ref(), user_id, &controls).await?;
        }
        "resume" => {
            controls.insert(task.clone(), "running".to_string());
            save_controls(db.as_ref(), user_id, &controls).await?;
        }
        "reset" => {
            reset_task(db.as_ref(), user_id, &task).await?;
        }
        _ => {
            return Err(AppError::BadRequest("Invalid action".to_string()));
        }
    }

    let state = controls
        .get(&task)
        .cloned()
        .unwrap_or_else(|| "running".to_string());

    Ok(Json(ProcessingControlResponse {
        task,
        action,
        state,
    }))
}

async fn reset_task(db: &DatabaseConnection, user_id: i32, task: &str) -> Result<()> {
    match task {
        "clip" => {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                DELETE FROM photo_embeddings pe
                USING photos p
                WHERE pe.photo_id = p.id AND p.user_id = $1 AND pe.modality = 'image'
                "#,
                vec![Value::from(user_id)],
            );
            db.execute(stmt)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to reset clip: {}", e)))?;
        }
        "face" => {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                DELETE FROM face_embeddings fe
                USING photos p
                WHERE fe.photo_id = p.id AND p.user_id = $1
                "#,
                vec![Value::from(user_id)],
            );
            db.execute(stmt)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to reset face: {}", e)))?;
        }
        "ocr" => {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Postgres,
                r#"
                UPDATE photos
                SET ml_result = COALESCE(ml_result, '{}'::jsonb) - 'ocr' - 'ocr_text'
                WHERE user_id = $1
                "#,
                vec![Value::from(user_id)],
            );
            db.execute(stmt)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to reset ocr: {}", e)))?;
        }
        "thumbnail" => {
            let stmt = Statement::from_sql_and_values(
                DbBackend::Postgres,
                "UPDATE photos SET thumbnail_url = NULL WHERE user_id = $1",
                vec![Value::from(user_id)],
            );
            db.execute(stmt)
                .await
                .map_err(|e| AppError::Internal(format!("Failed to reset thumbnail: {}", e)))?;
        }
        _ => {}
    }

    let stmt = Statement::from_sql_and_values(
        DbBackend::Postgres,
        r#"
        UPDATE photos
        SET ml_status = jsonb_set(COALESCE(ml_status, '{}'::jsonb), $2::text[], '"pending"'::jsonb)
        WHERE user_id = $1
        "#,
        vec![Value::from(user_id), Value::from(format!("{{{}}}", task))],
    );
    db.execute(stmt)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to reset status: {}", e)))?;

    Ok(())
}

#[derive(Debug)]
struct PendingPhoto {
    id: i32,
    user_id: i32,
    url: String,
    ml_status: Option<JsonValue>,
}

#[derive(Debug, sea_orm::FromQueryResult)]
struct PendingRow {
    id: i32,
    user_id: i32,
    url: String,
    ml_status: Option<JsonValue>,
}

pub async fn run_ml_planner_job(
    db: &DatabaseConnection,
    redis: &Arc<Mutex<redis::aio::ConnectionManager>>,
    config: &Config,
) -> Result<()> {
    let users = photos::Entity::find()
        .select_only()
        .column(photos::Column::UserId)
        .distinct()
        .into_tuple::<i32>()
        .all(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to list users: {}", e)))?;

    let batch_size = config.processing.planner_batch_size as i64;
    for user_id in users {
        let controls = load_controls(db, user_id).await?;
        if RUNNER_TASKS
            .iter()
            .all(|t| controls.get(*t).map(|v| v == "paused").unwrap_or(false))
        {
            continue;
        }

        let mut conditions: Vec<String> = Vec::new();
        for task in RUNNER_TASKS {
            if controls.get(task).map(|v| v == "paused").unwrap_or(false) {
                continue;
            }
            conditions.push(format!("(ml_status->>'{}' = 'pending')", task));
        }
        let where_clause = if conditions.is_empty() {
            "FALSE".to_string()
        } else {
            format!("({}) OR ml_status IS NULL", conditions.join(" OR "))
        };

        let sql = format!(
            r#"
            SELECT id, user_id, url, ml_status
            FROM photos
            WHERE user_id = $1 AND ({})
            ORDER BY id DESC
            LIMIT $2
            "#,
            where_clause
        );

        let stmt = Statement::from_sql_and_values(
            DbBackend::Postgres,
            &sql,
            vec![Value::from(user_id), Value::from(batch_size)],
        );
        let rows = PendingRow::find_by_statement(stmt)
            .all(db)
            .await
            .map_err(|e| AppError::Internal(format!("Failed to load pending: {}", e)))?;

        for row in rows {
            let pending = PendingPhoto {
                id: row.id,
                user_id: row.user_id,
                url: row.url,
                ml_status: row.ml_status,
            };
            enqueue_pending_photo(db, redis, config, &controls, pending).await?;
        }
    }

    Ok(())
}

async fn enqueue_pending_photo(
    db: &DatabaseConnection,
    redis: &Arc<Mutex<redis::aio::ConnectionManager>>,
    config: &Config,
    controls: &HashMap<String, String>,
    photo: PendingPhoto,
) -> Result<()> {
    let mut tasks = Vec::new();
    for task in RUNNER_TASKS {
        if controls
            .get(task)
            .map(|v| v == "paused")
            .unwrap_or(false)
        {
            continue;
        }
        let status = ml_status::get_task_status(&photo.ml_status, task);
        if status.as_deref() == Some("pending") || status.is_none() {
            tasks.push(task.to_string());
        }
    }

    if tasks.is_empty() {
        return Ok(());
    }

    let task_set: std::collections::HashSet<String> = tasks.iter().cloned().collect();
    let payload = serde_json::json!({
        "photo_id": photo.id,
        "user_id": photo.user_id,
        "path": resolve_photo_path(config, &photo.url),
        "tasks": tasks,
    });

    let serialized = serde_json::to_string(&payload)
        .map_err(|e| AppError::Internal(format!("Failed to serialize task: {}", e)))?;

    let mut conn = redis.lock().await;
    let _: i64 = conn
        .lpush(&config.redis.queue_name, serialized)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to enqueue task: {}", e)))?;

    let updates: Vec<(&str, &str)> = RUNNER_TASKS
        .iter()
        .filter(|t| task_set.contains(&t.to_string()))
        .map(|t| (*t, "queued"))
        .collect();

    let mut active: photos::ActiveModel = photos::Entity::find_by_id(photo.id)
        .one(db)
        .await
        .map_err(|e| AppError::Internal(format!("Database error: {}", e)))?
        .ok_or_else(|| AppError::NotFound("Photo not found".to_string()))?
        .into();

    active.ml_status = Set(Some(ml_status::merge_ml_status(
        photo.ml_status,
        &updates,
    )));
    active.updated_at = Set(chrono::Utc::now().naive_utc());
    active
        .update(db)
        .await
        .map_err(|e| AppError::Internal(format!("Failed to update ml_status: {}", e)))?;

    Ok(())
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
