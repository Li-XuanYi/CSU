use salvo::oapi::ToSchema;
use serde::Serialize;

/// 统一的 API 响应结构
#[derive(Serialize, Debug, ToSchema)]
pub struct ApiResponse<T> {
    /// 状态码：0 表示成功，其他值表示错误码
    pub code: i32,
    /// 错误信息：成功时为 null，失败时包含错误原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub msg: Option<String>,
    /// 响应数据：成功时包含数据，失败时为 null
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    /// 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            code: 0,
            msg: None,
            data: Some(data),
        }
    }

    /// 创建成功响应（无数据）
    pub fn success_empty() -> ApiResponse<()> {
        ApiResponse {
            code: 0,
            msg: None,
            data: None,
        }
    }

    /// 创建错误响应
    pub fn error(code: i32, msg: String) -> ApiResponse<()> {
        ApiResponse {
            code,
            msg: Some(msg),
            data: None,
        }
    }
}

/// 错误码���义
#[derive(Debug, Clone, Copy)]
pub enum ErrorCode {
    /// 数据库错误
    DatabaseError = 1001,
    /// 配置错误
    ConfigError = 1002,
    /// 未找到资源
    NotFound = 1003,
    /// 验证错误
    ValidationError = 1004,
    /// 内部错误
    InternalError = 1005,
    /// 参数错误
    BadRequest = 1006,
    /// 未授权
    Unauthorized = 1007,
    /// 权限不足
    Forbidden = 1008,
}

impl ErrorCode {
    pub fn code(&self) -> i32 {
        *self as i32
    }
}
