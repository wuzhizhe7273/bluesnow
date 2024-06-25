use crate::context::Context;
use crate::domain::{LoginRequest, RegisterRequest};
use crate::middleware::Claims;
use crate::utils;
use crate::{repo, Config};
use bluesnow_result::{Result, ToResult};
use sea_orm::prelude::Uuid;
use sea_orm::TransactionTrait;
use tracing::info;

/// 用户注册服务
pub async fn register(ctx: Context, req: RegisterRequest) -> Result<i32> {
    info!("Register a new user request:{req:?}.");
    let tx = ctx.db.begin().await?;
    repo::user::check_unique_by_username(&tx, &req.username).await?;
    repo::user::check_unique_by_email(&tx, &req.email).await?;
    let user_id = repo::user::save(&tx, &req.username, &req.password, &req.email).await?;
    tx.commit().await?;
    Ok(user_id)
}

/// 用户登录服务
pub async fn login(ctx: Context, req: LoginRequest) -> Result<String> {
    info!("User login request:{req:?}");
    // 插寻用户
    let user = repo::user::find_by_email(&ctx.db, &req.email)
        .await?
        .to_result()?;
    // 验证密码
    utils::pwd::verify(req.password, user.password).await?;
    let config = Config::get()?;
    // 生成token
    let token = Claims::new(user.id, config.jwt.get_exp()).encode(config.jwt.get_encoding_key())?;
    Ok(token)
}
