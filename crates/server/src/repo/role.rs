use bluesnow_result::{Result, ToResult};
use sea_orm::{ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, QueryFilter};
pub async fn get_by_id<C>(conn: &C, id: i32) -> Result<entity::role::Model>
where
    C: ConnectionTrait,
{
    let role = entity::role::Entity::find_by_id(id)
        .one(conn)
        .await?
        .to_result()?;
    Ok(role)
}

pub async fn get_by_name<C>(conn: &C, name: &str) -> Result<entity::role::Model>
where
    C: ConnectionTrait,
{
    let role = entity::role::Entity::find()
        .filter(entity::role::Column::Name.eq(name))
        .one(conn)
        .await?
        .to_result()?;
    Ok(role)
}
pub async fn get_related_api<C>(
    conn: &C,
    role: entity::role::Model,
) -> Result<Vec<entity::api::Model>>
where
    C: ConnectionTrait,
{
    let apis = role.find_related(entity::api::Entity).all(conn).await?;
    Ok(apis)
}
