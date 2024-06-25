use crate::context::Context;
use crate::repo;
use crate::Config;
use axum::extract::{FromRequestParts, OriginalUri, Request, State};
use axum::http::request::Parts;
use axum::middleware::Next;
use axum::response::Response;
use axum::{RequestExt, RequestPartsExt};
use axum_extra::headers::authorization::Bearer;
use axum_extra::headers::Authorization;
use axum_extra::TypedHeader;
use bluesnow_result::{Error, Result};
use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::ConnectionTrait;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;
use std::time::Duration;

pub static DECODE_HEADER: OnceLock<Validation> = OnceLock::new();
pub static ENCODE_HEADER: OnceLock<Header> = OnceLock::new();
#[derive(Deserialize, Serialize, Clone)]
pub struct Claims {
    pub sub: i32,
    exp: u64,
    iat: u64,
}
impl Claims {
    pub fn new(sub: i32, exp: Duration) -> Self {
        let iat = Utc::now().timestamp() as u64;
        let exp = iat + exp.as_secs();
        Self { sub, exp, iat }
    }
    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> std::result::Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        let decode_header = DECODE_HEADER.get_or_init(|| Validation::new(Algorithm::RS256));
        jsonwebtoken::decode::<Self>(token, key, &decode_header)
    }
    pub fn encode(
        &self,
        key: &EncodingKey,
    ) -> std::result::Result<String, jsonwebtoken::errors::Error> {
        let encode_header = ENCODE_HEADER.get_or_init(|| Header::new(Algorithm::HS256));
        jsonwebtoken::encode(&encode_header, self, key)
    }
}
#[axum::async_trait]
impl FromRequestParts<Context> for Claims {
    type Rejection = Error;
    async fn from_request_parts(
        parts: &mut Parts,
        _state: &Context,
    ) -> std::result::Result<Self, Self::Rejection> {
        let bearer = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await?;
        let decoding_key = Config::get()?.jwt.get_decoding_key();
        let claims = Claims::decode(bearer.token(), decoding_key)?.claims;
        Ok(claims)
    }
}
pub async fn auth(State(ctx): State<Context>, request: Request, next: Next) -> Result<Response> {
    // 获取访问路径
    let path = if let Some(path) = request.extensions().get::<OriginalUri>() {
        path.0.path().to_owned()
    } else {
        request.uri().path().to_owned()
    };
    let method = request.method().to_string();
    let (mut parts, body) = request.into_parts();
    match parts.extract_with_state::<Claims, _>(&ctx).await {
        // 成功解码claims的情况
        Ok(claims) => {
            // 根据id获取用户
            let user = repo::user::get_by_id(&ctx.db, claims.sub).await?;
            // 如果是超级用户则直接放行
            if user.is_super_user {
                let req = Request::from_parts(parts, body);
                return Ok(next.run(req).await);
            } else
            // 否则根据具体角色检验
            {
                let apis = repo::user::get_related_api(&ctx.db, user).await?;
                // 有效则放行
                if apis
                    .iter()
                    .any(|api| api.path == path && api.method == method)
                {
                    let mut req = Request::from_parts(parts, body);
                    req.extensions_mut().insert(claims);
                    return Ok(next.run(req).await);
                } else
                // 无效则抛出Forbidden错误
                {
                    return Err(Error::PermissionDenied(format!(
                        "你没有api:{}的访问权限",
                        path
                    )));
                }
            }
        }
        // 无法成功解码claims
        Err(e) => {
            println!("{:#?}", e);
            let guest_apis = get_guest_api(&ctx.db).await?;
            // 如果当前api在访客api中则放行
            println!("{:#?}", guest_apis);
            if guest_apis
                .iter()
                .any(|api| api.path == path && api.method == method)
            {
                let req = Request::from_parts(parts, body);
                return Ok(next.run(req).await);
            } else
            // 否则抛出错误
            {
                return Err(e);
            }
        }
    }
}
async fn get_guest_api<C>(conn: &C) -> Result<Vec<entity::api::Model>>
where
    C: ConnectionTrait,
{
    let guest = repo::role::get_by_name(conn, "Guest").await?;
    let apis = repo::role::get_related_api(conn, guest).await?;
    Ok(apis)
}
