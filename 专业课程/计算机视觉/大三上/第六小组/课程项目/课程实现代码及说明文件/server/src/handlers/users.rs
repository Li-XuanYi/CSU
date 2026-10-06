use crate::models::users::{ActiveModel as ActiveUser, Entity as Users, Model as User};
use crate::response::ApiResponse;
use crate::utils::jwt::JwtConfig;
use crate::utils::password::{hash_password, verify_password};
use salvo::oapi::{ToSchema, endpoint, extract::JsonBody};
use salvo::prelude::*;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    Set,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, ToSchema)]
pub struct UsersListResponse {
    users: Vec<User>,
    total: usize,
}

#[derive(Deserialize, ToSchema)]
pub struct RegisterRequest {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub user: UserInfo,
}

#[derive(Serialize, ToSchema)]
pub struct UserInfo {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub role: String,
}

#[derive(Deserialize, ToSchema)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

#[derive(Deserialize, ToSchema)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

/// 获取用户列表
#[endpoint(
    tags("Users"),
    responses(
        (status_code = 200, description = "成功获取用户列表", body = ApiResponse<UsersListResponse>),
        (status_code = 401, description = "未授权"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn list_users(
    depot: &mut Depot,
) -> Result<Json<ApiResponse<UsersListResponse>>, crate::error::AppError> {
    // 从 depot 获取数据库连接
    let db = depot.obtain::<Arc<DatabaseConnection>>().map_err(|_| {
        crate::error::AppError::Internal("Database connection not found".to_string())
    })?;

    // 查询所有用户
    let users = Users::find().all(db.as_ref()).await?;

    let total = users.len();

    Ok(Json(ApiResponse::success(UsersListResponse {
        users,
        total,
    })))
}

/// 用户注册
#[endpoint(
    tags("Auth"),
    request_body = RegisterRequest,
    responses(
        (status_code = 200, description = "注册成功", body = ApiResponse<AuthResponse>),
        (status_code = 400, description = "请求参数错误"),
        (status_code = 422, description = "验证失败"),
    )
)]
pub async fn register(
    body: JsonBody<RegisterRequest>,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<AuthResponse>>, crate::error::AppError> {
    let db = depot.obtain::<Arc<DatabaseConnection>>().map_err(|_| {
        crate::error::AppError::Internal("Database connection not found".to_string())
    })?;

    let register_req = body.into_inner();

    // 验证输入
    if register_req.username.is_empty()
        || register_req.email.is_empty()
        || register_req.password.is_empty()
    {
        return Err(crate::error::AppError::Validation(
            "Username, email and password are required".to_string(),
        ));
    }

    if register_req.password.len() < 6 {
        return Err(crate::error::AppError::Validation(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    // 检查邮箱是否已存在（username 可以重复）
    let existing_email = Users::find()
        .filter(crate::models::users::Column::Email.eq(&register_req.email))
        .one(db.as_ref())
        .await?;

    if existing_email.is_some() {
        return Err(crate::error::AppError::Validation(
            "Email already exists".to_string(),
        ));
    }

    // 检查是否是首个用户，如果是则设为管理员
    let user_count = Users::find().count(db.as_ref()).await?;
    let role = if user_count == 0 {
        "admin".to_string()
    } else {
        "user".to_string()
    };

    // 哈希密码
    let hashed_password = hash_password(&register_req.password)?;

    // 创建用户
    let jwt_config = JwtConfig::default();
    let now = chrono::Utc::now().naive_utc();

    let new_user = ActiveUser {
        username: Set(register_req.username.clone()),
        email: Set(register_req.email.clone()),
        password: Set(hashed_password),
        role: Set(role.clone()),
        refresh_token: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = new_user.insert(db.as_ref()).await?;

    // 生成 tokens
    let access_token = jwt_config.generate_access_token(user.id, &user.username, &role)?;
    let refresh_tok = jwt_config.generate_refresh_token(user.id, &user.username, &role)?;

    // 保存 refresh token
    let mut user_active: ActiveUser = user.clone().into();
    user_active.refresh_token = Set(Some(refresh_tok.clone()));
    user_active.update(db.as_ref()).await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        access_token,
        refresh_token: refresh_tok,
        user: UserInfo {
            id: user.id,
            username: user.username,
            email: user.email,
            role: role,
        },
    })))
}

/// 用户登录
#[endpoint(
    tags("Auth"),
    request_body = LoginRequest,
    responses(
        (status_code = 200, description = "登录成功", body = ApiResponse<AuthResponse>),
        (status_code = 401, description = "邮箱或密码错误"),
    )
)]
pub async fn login(
    body: JsonBody<LoginRequest>,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<AuthResponse>>, crate::error::AppError> {
    let db = depot.obtain::<Arc<DatabaseConnection>>().map_err(|_| {
        crate::error::AppError::Internal("Database connection not found".to_string())
    })?;

    let login_req = body.into_inner();

    // 使用邮箱查找用户
    let user = Users::find()
        .filter(crate::models::users::Column::Email.eq(&login_req.email))
        .one(db.as_ref())
        .await?
        .ok_or_else(|| {
            crate::error::AppError::Unauthorized("Invalid email or password".to_string())
        })?;

    // 验证密码
    if !verify_password(&login_req.password, &user.password)? {
        return Err(crate::error::AppError::Unauthorized(
            "Invalid email or password".to_string(),
        ));
    }

    // 生成 tokens
    let jwt_config = JwtConfig::default();
    let access_token = jwt_config.generate_access_token(user.id, &user.username, &user.role)?;
    let refresh_tok = jwt_config.generate_refresh_token(user.id, &user.username, &user.role)?;

    // 保存 refresh token
    let mut user_active: ActiveUser = user.clone().into();
    user_active.refresh_token = Set(Some(refresh_tok.clone()));
    user_active.update(db.as_ref()).await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        access_token,
        refresh_token: refresh_tok,
        user: UserInfo {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
        },
    })))
}

/// 刷新 Token
#[endpoint(
    tags("Auth"),
    request_body = RefreshTokenRequest,
    responses(
        (status_code = 200, description = "刷新成功", body = ApiResponse<AuthResponse>),
        (status_code = 401, description = "Refresh Token 无效或过期"),
    )
)]
pub async fn refresh_token(
    body: JsonBody<RefreshTokenRequest>,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<AuthResponse>>, crate::error::AppError> {
    let db = depot.obtain::<Arc<DatabaseConnection>>().map_err(|_| {
        crate::error::AppError::Internal("Database connection not found".to_string())
    })?;

    let refresh_req = body.into_inner();

    // 验证 refresh token
    let jwt_config = JwtConfig::default();
    let claims = jwt_config.verify_refresh_token(&refresh_req.refresh_token)?;

    let user_id: i32 = claims
        .sub
        .parse()
        .map_err(|_| crate::error::AppError::Internal("Invalid user ID in token".to_string()))?;

    // 查找用户并验证 refresh token
    let user = Users::find_by_id(user_id)
        .one(db.as_ref())
        .await?
        .ok_or_else(|| crate::error::AppError::Unauthorized("User not found".to_string()))?;

    // 验证 refresh token 是否匹配
    if user.refresh_token.as_ref() != Some(&refresh_req.refresh_token) {
        return Err(crate::error::AppError::Unauthorized(
            "Invalid refresh token".to_string(),
        ));
    }

    // 生成新的 tokens
    let new_access_token = jwt_config.generate_access_token(user.id, &user.username, &user.role)?;
    let new_refresh_token =
        jwt_config.generate_refresh_token(user.id, &user.username, &user.role)?;

    // 更新 refresh token
    let mut user_active: ActiveUser = user.clone().into();
    user_active.refresh_token = Set(Some(new_refresh_token.clone()));
    user_active.update(db.as_ref()).await?;

    Ok(Json(ApiResponse::success(AuthResponse {
        access_token: new_access_token,
        refresh_token: new_refresh_token,
        user: UserInfo {
            id: user.id,
            username: user.username.clone(),
            email: user.email.clone(),
            role: user.role.clone(),
        },
    })))
}

/// 管理员创建用户
#[endpoint(
    tags("Admin"),
    request_body = CreateUserRequest,
    responses(
        (status_code = 200, description = "创建成功", body = ApiResponse<UserInfo>),
        (status_code = 401, description = "未授权"),
        (status_code = 403, description = "权限不足"),
        (status_code = 422, description = "验证失败"),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_user(
    body: JsonBody<CreateUserRequest>,
    depot: &mut Depot,
) -> Result<Json<ApiResponse<UserInfo>>, crate::error::AppError> {
    let db = depot.obtain::<Arc<DatabaseConnection>>().map_err(|_| {
        crate::error::AppError::Internal("Database connection not found".to_string())
    })?;

    let create_req = body.into_inner();

    // 验证输入
    if create_req.username.is_empty()
        || create_req.email.is_empty()
        || create_req.password.is_empty()
    {
        return Err(crate::error::AppError::Validation(
            "Username, email and password are required".to_string(),
        ));
    }

    if create_req.password.len() < 6 {
        return Err(crate::error::AppError::Validation(
            "Password must be at least 6 characters".to_string(),
        ));
    }

    // 检查邮箱是否已存在（username 可以重复）
    let existing_email = Users::find()
        .filter(crate::models::users::Column::Email.eq(&create_req.email))
        .one(db.as_ref())
        .await?;

    if existing_email.is_some() {
        return Err(crate::error::AppError::Validation(
            "Email already exists".to_string(),
        ));
    }

    // 验证角色
    let role = create_req.role.unwrap_or_else(|| "user".to_string());
    if role != "admin" && role != "user" {
        return Err(crate::error::AppError::Validation(
            "Invalid role. Must be 'admin' or 'user'".to_string(),
        ));
    }

    // 哈希密码
    let hashed_password = hash_password(&create_req.password)?;

    // 创建用户
    let now = chrono::Utc::now().naive_utc();
    let new_user = ActiveUser {
        username: Set(create_req.username.clone()),
        email: Set(create_req.email.clone()),
        password: Set(hashed_password),
        role: Set(role.clone()),
        refresh_token: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
        ..Default::default()
    };

    let user = new_user.insert(db.as_ref()).await?;

    Ok(Json(ApiResponse::success(UserInfo {
        id: user.id,
        username: user.username,
        email: user.email,
        role: user.role,
    })))
}
