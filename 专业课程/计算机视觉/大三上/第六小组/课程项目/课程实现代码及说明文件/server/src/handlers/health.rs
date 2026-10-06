use crate::response::ApiResponse;
use salvo::prelude::*;
use serde::Serialize;

#[derive(Serialize)]
pub struct HealthData {
    status: String,
}

/// 健康检查接口
#[handler]
pub async fn health_check() -> Json<ApiResponse<HealthData>> {
    Json(ApiResponse::success(HealthData {
        status: "ok".to_string(),
    }))
}
