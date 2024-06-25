use bluesnow_result::{Result, ToResult};
use chrono::Utc;
use entity::category::Model;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, IntoActiveModel, QueryFilter,
};

pub async fn create<C>(
    coon: &C,
    p_id: Option<i32>,
    name: &str,
    desc: &str,
) -> Result<entity::category::Model>
where
    C: ConnectionTrait,
{
    let category = entity::category::ActiveModel {
        p_id: Set(p_id),
        name: Set(name.to_string()),
        desc: Set(desc.to_string()),
        ..Default::default()
    }
    .insert(coon)
    .await?;
    Ok(category)
}

pub async fn check_exist_by_id<C>(coon: &C, id: i32) -> Result<bool>
where
    C: ConnectionTrait,
{
    Ok(entity::category::Entity::find()
        .filter(entity::category::Column::Id.eq(id))
        .one(coon)
        .await?
        .is_some())
}
pub async fn check_unique_by_name<C>(conn: &C, name: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    entity::category::Entity::find()
        .filter(entity::category::Column::Name.eq(name))
        .one(conn)
        .await?
        .check_absent_details(vec![("name".to_string(), name.to_string())])
}

pub async fn list<C>(conn: &C) -> Result<Vec<Model>>
where
    C: ConnectionTrait,
{
    let models = entity::category::Entity::find().all(conn).await?;
    Ok(models)
}

pub async fn update<C>(coon: &C, id: i32, p_id: Option<i32>, name: &str, desc: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    let mut cate = entity::category::Entity::find_by_id(id)
        .one(coon)
        .await?
        .to_result()?
        .into_active_model();
    cate.p_id = Set(p_id);
    cate.name = Set(name.to_string());
    cate.desc = Set(desc.to_string());
    cate.updated_at = Set(Utc::now().fixed_offset());
    entity::category::Entity::update(cate);
    Ok(())
}

pub async fn delete<C>(coon: &C, id: i32) -> Result<()>
where
    C: ConnectionTrait,
{
    entity::category::Entity::delete_by_id(id)
        .exec(coon)
        .await?;
    Ok(())
}
