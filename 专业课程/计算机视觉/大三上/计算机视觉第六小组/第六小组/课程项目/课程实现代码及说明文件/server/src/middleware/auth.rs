use crate::error::AppError;
use crate::utils::jwt::JwtConfig;
use salvo::prelude::*;

/// 从 Authorization header 中提取 token
fn extract_token(req: &Request) -> Result<String, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("Missing authorization header".to_string()))?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized(
            "Invalid authorization header format".to_string(),
        ));
    }

    Ok(auth_header[7..].to_string())
}

/// JWT 认证中间件
#[handler]
pub async fn jwt_auth(
    req: &mut Request,
    depot: &mut Depot,
    res: &mut Response,
    ctrl: &mut FlowCtrl,
) {
    let token = match extract_token(req) {
        Ok(t) => t,
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(crate::response::ApiResponse::<()>::error(
                crate::response::ErrorCode::Unauthorized.code(),
                e.to_string(),
            )));
            ctrl.skip_rest();
            return;
        }
    };

    let jwt_config = JwtConfig::default();
    match jwt_config.verify_access_token(&token) {
        Ok(claims) => {
            // 将用户信息存入 depot 供后续 handler 使用
            depot.insert("user_id", claims.sub.clone());
            depot.insert("username", claims.username.clone());
            depot.insert("user_role", claims.role.clone());
            depot.insert("claims", claims);
        }
        Err(e) => {
            res.status_code(StatusCode::UNAUTHORIZED);
            res.render(Json(crate::response::ApiResponse::<()>::error(
                crate::response::ErrorCode::Unauthorized.code(),
                e.to_string(),
            )));
            ctrl.skip_rest();
        }
    }
}

/// 管理员权限中间件
#[handler]
pub async fn admin_only(depot: &mut Depot, res: &mut Response, ctrl: &mut FlowCtrl) {
    let user_role = depot.get::<String>("user_role").ok();

    match user_role {
        Some(role) if role == "admin" => {
            // 用户是管理员，继续
        }
        _ => {
            res.status_code(StatusCode::FORBIDDEN);
            res.render(Json(crate::response::ApiResponse::<()>::error(
                crate::response::ErrorCode::Forbidden.code(),
                "Admin access required".to_string(),
            )));
            ctrl.skip_rest();
        }
    }
}
