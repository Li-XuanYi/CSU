use crate::error::AppError;
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,        // 用户ID
    pub username: String,   // 用户名
    pub role: String,       // 用户角色
    pub exp: i64,           // 过期时间
    pub iat: i64,           // 签发时间
    pub token_type: String, // token类型: "access" 或 "refresh"
}

pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: i64,  // 秒
    pub refresh_token_expiry: i64, // 秒
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: std::env::var("JWT_SECRET")
                .unwrap_or_else(|_| "your-secret-key-change-this-in-production".to_string()),
            access_token_expiry: 3600,    // 1小时
            refresh_token_expiry: 604800, // 7天
        }
    }
}

impl JwtConfig {
    /// 生成 access token
    pub fn generate_access_token(
        &self,
        user_id: i32,
        username: &str,
        role: &str,
    ) -> Result<String, AppError> {
        self.generate_token(user_id, username, role, "access", self.access_token_expiry)
    }

    /// 生成 refresh token
    pub fn generate_refresh_token(
        &self,
        user_id: i32,
        username: &str,
        role: &str,
    ) -> Result<String, AppError> {
        self.generate_token(
            user_id,
            username,
            role,
            "refresh",
            self.refresh_token_expiry,
        )
    }

    /// 生成 token
    fn generate_token(
        &self,
        user_id: i32,
        username: &str,
        role: &str,
        token_type: &str,
        expiry: i64,
    ) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = (now + Duration::seconds(expiry)).timestamp();

        let claims = Claims {
            sub: user_id.to_string(),
            username: username.to_string(),
            role: role.to_string(),
            exp,
            iat: now.timestamp(),
            token_type: token_type.to_string(),
        };

        encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.secret.as_bytes()),
        )
        .map_err(|e| AppError::Internal(format!("Failed to generate token: {}", e)))
    }

    /// 验证并解析 token
    pub fn verify_token(&self, token: &str) -> Result<Claims, AppError> {
        decode::<Claims>(
            token,
            &DecodingKey::from_secret(self.secret.as_bytes()),
            &Validation::default(),
        )
        .map(|data| data.claims)
        .map_err(|e| AppError::Unauthorized(format!("Invalid token: {}", e)))
    }

    /// 验证 access token
    pub fn verify_access_token(&self, token: &str) -> Result<Claims, AppError> {
        let claims = self.verify_token(token)?;

        if claims.token_type != "access" {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        Ok(claims)
    }

    /// 验证 refresh token
    pub fn verify_refresh_token(&self, token: &str) -> Result<Claims, AppError> {
        let claims = self.verify_token(token)?;

        if claims.token_type != "refresh" {
            return Err(AppError::Unauthorized("Invalid token type".to_string()));
        }

        Ok(claims)
    }
}
