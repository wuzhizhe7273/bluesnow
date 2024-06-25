use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use entity::utils::Resource;
use sea_orm::DbErr;
use serde::Serialize;
use std::io;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("{0} not found ")]
    NotFound(Resource),
    #[error("{0} already exists")]
    ResourceExist(Resource),
    #[error("{0}")]
    PermissionDenied(String),
    #[error("{0}")]
    Conflict(String),
    #[error("{0}")]
    Unauthorized(String),
    #[error("bad request{0}")]
    BadRequest(String),
    #[error("{0}")]
    Hash(String),
    #[error(transparent)]
    InvalidInput(#[from] garde::Report),
    #[error(transparent)]
    Db(#[from] DbErr),
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    Jwt(#[from] jsonwebtoken::errors::Error),
    #[error(transparent)]
    Config(#[from] figment::Error),
    #[error(transparent)]
    ParseJson(#[from] serde_json::Error),
    #[error(transparent)]
    Cli(#[from] clap::Error),
    #[error(transparent)]
    SpawnTask(#[from] tokio::task::JoinError),
    #[error(transparent)]
    Unknown(#[from] anyhow::Error),
    #[error(transparent)]
    Axum(#[from] axum::Error),
    #[error(transparent)]
    TypeHeader(#[from] axum_extra::typed_header::TypedHeaderRejection),
}

impl Error {
    pub fn response(self) -> (StatusCode, ResponseError) {
        use Error::*;
        let message = self.to_string();
        let (status, code, details) = match self {
            BadRequest(_err) => (StatusCode::BAD_REQUEST, "BAD_REQUEST", vec![]),
            NotFound(resource) => (StatusCode::NOT_FOUND, "NOT_FOUND", resource.details.clone()),
            ResourceExist(resource) => (
                StatusCode::CONFLICT,
                "RESOURCE_ALREADY_EXISTS",
                resource.details.clone(),
            ),
            PermissionDenied(_err) => (StatusCode::FORBIDDEN, "PERMISSION_DENIED", vec![]),
            Conflict(_err) => (StatusCode::CONFLICT, "CONFLICT", vec![]),
            Unauthorized(_err) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", vec![]),
            Jwt(_err) => (StatusCode::UNAUTHORIZED, "UNAUTHORIZED", vec![]),
            Hash(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "HASH", vec![]),
            InvalidInput(_err) => (
                StatusCode::BAD_REQUEST,
                "INVALID_INPUT",
                _err.iter()
                    .map(|(p, e)| (p.to_string(), e.to_string()))
                    .collect(),
            ),
            Db(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "DATABASE", vec![]),
            Axum(_err) => (StatusCode::INTERNAL_SERVER_ERROR, "AXUM_ERROR", vec![]),
            TypeHeader(_err) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "TYPE_HEADER_ERROR",
                vec![],
            ),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_SERVER", vec![]),
        };
        (status, ResponseError::new(status, code, message, details))
    }
}
impl IntoResponse for Error {
    fn into_response(self) -> Response {
        let (status, body) = self.response();
        (status, Json(body)).into_response()
    }
}

#[derive(Serialize)]
pub struct ResponseError {
    pub status: u16,
    pub code: String,
    pub message: String,
    pub details: Vec<(String, String)>,
}

impl ResponseError {
    pub fn new(
        status: StatusCode,
        code: impl Into<String>,
        message: impl Into<String>,
        details: Vec<(String, String)>,
    ) -> Self {
        Self {
            status: status.as_u16(),
            code: code.into(),
            message: message.into(),
            details,
        }
    }
}
impl From<argon2::password_hash::Error> for Error {
    fn from(value: argon2::password_hash::Error) -> Self {
        Error::Hash(value.to_string())
    }
}
pub fn invalid_input_error(field: &'static str, message: &'static str) -> Error {
    let mut report = garde::Report::new();
    report.append(garde::Path::new(field), garde::Error::new(message));
    Error::InvalidInput(report)
}
