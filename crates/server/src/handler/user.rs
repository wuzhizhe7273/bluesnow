use crate::context::Context;
use crate::domain::{LoginRequest, LoginResponse, RegisterRequest, RegisterResponse};
use crate::service;
use axum::extract::State;
use axum::Json;
use bluesnow_result::Result;
use garde::Validate;
use tracing::{info, warn};

/// 用户注册
pub async fn register(
    State(ctx): State<Context>,
    Json(req): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>> {
    info!("Register new user with request: {req:?}");
    req.validate()?;
    match service::user::register(ctx, req).await {
        Ok(user_id) => {
            info!("Successfully register user: {user_id}");
            let resp = RegisterResponse { id: user_id };
            Ok(Json(resp))
        }
        Err(e) => {
            warn!("Unsuccessfully register user: {e:?}");
            Err(e)
        }
    }
}

/// 用户登录
pub async fn login(
    State(ctx): State<Context>,
    Json(req): Json<LoginRequest>,
) -> Result<Json<LoginResponse>> {
    match service::user::login(ctx, req).await {
        Ok(token) => {
            let resp = LoginResponse { token };
            Ok(Json(resp))
        }
        Err(e) => Err(e),
    }
}
