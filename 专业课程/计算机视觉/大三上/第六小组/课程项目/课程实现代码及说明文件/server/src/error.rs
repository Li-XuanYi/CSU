use crate::response::{ApiResponse, ErrorCode};
use salvo::oapi::EndpointOutRegister;
use salvo::prelude::*;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Bad request: {0}")]
    BadRequest(String),

    #[error("Unauthorized: {0}")]
    Unauthorized(String),

    #[error("Forbidden: {0}")]
    Forbidden(String),
}

impl AppError {
    /// 获取错误码
    pub fn error_code(&self) -> ErrorCode {
        match self {
            AppError::Database(_) => ErrorCode::DatabaseError,
            AppError::Config(_) => ErrorCode::ConfigError,
            AppError::NotFound(_) => ErrorCode::NotFound,
            AppError::Validation(_) => ErrorCode::ValidationError,
            AppError::Internal(_) => ErrorCode::InternalError,
            AppError::BadRequest(_) => ErrorCode::BadRequest,
            AppError::Unauthorized(_) => ErrorCode::Unauthorized,
            AppError::Forbidden(_) => ErrorCode::Forbidden,
        }
    }

    /// 转换为 API 响应
    pub fn to_response<T>(&self) -> ApiResponse<T> {
        ApiResponse {
            code: self.error_code().code(),
            msg: Some(self.to_string()),
            data: None,
        }
    }
}

/// 实现 Salvo Writer trait 以支持错误自动转换为 HTTP 响应
#[async_trait::async_trait]
impl Writer for AppError {
    async fn write(mut self, _req: &mut Request, _depot: &mut Depot, res: &mut Response) {
        let status_code = match self.error_code() {
            ErrorCode::NotFound => StatusCode::NOT_FOUND,
            ErrorCode::ValidationError | ErrorCode::BadRequest => StatusCode::BAD_REQUEST,
            ErrorCode::Unauthorized => StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => StatusCode::FORBIDDEN,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        res.status_code(status_code);
        res.render(Json(self.to_response::<()>()));
    }
}

/// 实现 EndpointOutRegister 以支持 OpenAPI
impl EndpointOutRegister for AppError {
    fn register(
        _components: &mut salvo::oapi::Components,
        _operation: &mut salvo::oapi::Operation,
    ) {
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
