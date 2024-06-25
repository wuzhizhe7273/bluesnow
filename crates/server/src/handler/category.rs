use crate::context::Context;
use crate::domain::{
    CategoryListResponseNode, CreateCategoryRequest, CreateCategoryResponse, UpdateCategoryRequest,
};
use crate::service;
use crate::utils::tree::Tree;
use axum::extract::{Path, State};
use axum::Json;
use bluesnow_result::Result;

pub async fn create(
    State(ctx): State<Context>,
    Json(req): Json<CreateCategoryRequest>,
) -> Result<Json<CreateCategoryResponse>> {
    match service::category::create(ctx, req).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => return Err(e),
    }
}
pub async fn list(State(ctx): State<Context>) -> Result<Json<Tree<CategoryListResponseNode>>> {
    match service::category::list(ctx).await {
        Ok(resp) => Ok(Json(resp)),
        Err(e) => return Err(e),
    }
}

pub async fn update(
    State(ctx): State<Context>,
    Path(id): Path<i32>,
    Json(req): Json<UpdateCategoryRequest>,
) -> Result<()> {
    service::category::update(ctx, id, req).await
}

pub async fn delete(State(ctx): State<Context>, Path(id): Path<i32>) -> Result<()> {
    service::category::delete(ctx, id).await
}
