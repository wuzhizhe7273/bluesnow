use crate::context::Context;
use crate::domain::{
    CategoryListResponseNode, CreateCategoryRequest, CreateCategoryResponse, UpdateCategoryRequest,
};
use crate::repo;
use crate::utils::tree::Tree;
use bluesnow_result::{invalid_input_error, Result};
use sea_orm::TransactionTrait;

/// 创建新的分类
pub async fn create(ctx: Context, req: CreateCategoryRequest) -> Result<CreateCategoryResponse> {
    let conn = ctx.db;
    let tx = conn.begin().await?;
    // 检查父分类是否存在
    if let Some(p_id) = req.p_id {
        if !repo::category::check_exist_by_id(&tx, p_id).await? {
            return Err(invalid_input_error("pId", "父分类不存在"));
        };
    }
    // 检查分类名是否唯一
    repo::category::check_unique_by_name(&tx, &req.name).await?;
    // 创建分类
    let cate = repo::category::create(&tx, req.p_id, &req.name, &req.desc).await?;
    tx.commit().await?;
    Ok(CreateCategoryResponse::new(cate.id))
}

pub async fn list(ctx: Context) -> Result<Tree<CategoryListResponseNode>> {
    let conn = ctx.db;
    let list = repo::category::list(&conn).await?;
    Ok(list.into())
}

pub async fn update(ctx: Context, id: i32, req: UpdateCategoryRequest) -> Result<()> {
    let conn = ctx.db;
    let tx = conn.begin().await?;
    if !repo::category::check_exist_by_id(&tx, id).await? {
        return Err(invalid_input_error("id", "category is not exists"));
    };
    if !repo::category::check_exist_by_id(&tx, id).await? {
        return Err(invalid_input_error("pId", "category is not exists"));
    }
    repo::category::check_unique_by_name(&tx, &req.name).await?;
    repo::category::update(&tx, id, req.p_id, &req.name, &req.desc).await?;
    tx.commit().await?;
    Ok(())
}

pub async fn delete(ctx: Context, id: i32) -> Result<()> {
    let conn = ctx.db;
    let tx = conn.begin().await?;
    if !repo::category::check_exist_by_id(&tx, id).await? {
        return Err(invalid_input_error("id", "category is not exists"));
    };
    repo::category::delete(&tx, id).await?;
    tx.commit().await?;
    Ok(())
}
