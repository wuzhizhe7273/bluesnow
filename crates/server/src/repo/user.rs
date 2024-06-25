use crate::utils;
use bluesnow_result::{Result, ToResult};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, EntityTrait, ModelTrait, QueryFilter,
};

pub async fn check_unique_by_username<C>(conn: &C, username: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(username))
        .one(conn)
        .await?
        .check_absent_details(vec![("username".to_string(), username.to_string())])
}
pub async fn check_unique_by_email<C>(conn: &C, email: &str) -> Result<()>
where
    C: ConnectionTrait,
{
    entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(conn)
        .await?
        .check_absent_details(vec![("email".to_string(), email.to_string())])
}

pub async fn save<C>(conn: &C, username: &str, password: &str, email: &str) -> Result<i32>
where
    C: ConnectionTrait,
{
    let user = entity::user::ActiveModel {
        username: Set(username.to_string()),
        password: Set(utils::pwd::hash(password).await?),
        email: Set(email.to_string()),
        ..Default::default()
    }
    .insert(conn)
    .await?;
    Ok(user.id)
}

pub async fn find_by_email<C>(conn: &C, email: &str) -> Result<Option<entity::user::Model>>
where
    C: ConnectionTrait,
{
    let model = entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(conn)
        .await?;
    Ok(model)
}
pub async fn check_exist_by_email<C>(conn: &C, email: &str) -> Result<bool>
where
    C: ConnectionTrait,
{
    Ok(entity::user::Entity::find()
        .filter(entity::user::Column::Email.eq(email))
        .one(conn)
        .await?
        .is_some())
}

pub async fn get_by_id<C>(conn: &C, id: i32) -> Result<entity::user::Model>
where
    C: ConnectionTrait,
{
    let user = entity::user::Entity::find_by_id(id)
        .one(conn)
        .await?
        .to_result()?;
    Ok(user)
}

pub async fn get_by_username<C>(conn: &C, username: &str) -> Result<entity::user::Model> 
where 
    C:ConnectionTrait
{
    let user = entity::user::Entity::find()
        .filter(entity::user::Column::Username.eq(username))
        .one(conn)
        .await?
        .to_result()?;
    Ok(user)
}

pub async fn get_related_api<C>(
    conn: &C,
    user: entity::user::Model,
) -> Result<Vec<entity::api::Model>>
where
    C: ConnectionTrait,
{
    let apis = user.find_linked(entity::utils::UserToApi).all(conn).await?;
    Ok(apis)
}
