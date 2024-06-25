use crate::config::Config;
use crate::param::DbInitParam;
use crate::{repo, utils};
use bluesnow_result::Error;
use migration::{IntoSchemaManagerConnection, Migrator, MigratorTrait};
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ConnectOptions, ConnectionTrait, Database, DbConn, DbErr, EntityTrait,
    Insert, IntoActiveModel,
};
use serde::Deserialize;

pub async fn connect(config: &Config) -> Result<DbConn, DbErr> {
    let url = config.db.get_url();
    let opt = ConnectOptions::new(url);
    let conn = Database::connect(opt).await?;
    Ok(conn)
}

pub async fn seed<A, C>(db: &C, value: &str) -> bluesnow_result::Result<()>
where
    A: ActiveModelTrait + Send + Sync,
    <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: IntoActiveModel<A>,
    for<'de> <<A as ActiveModelTrait>::Entity as EntityTrait>::Model: Deserialize<'de>,
    Insert<A>: Send + Sync,
    C: ConnectionTrait,
{
    let loader: Vec<serde_json::Value> = serde_json::from_str(value)?;
    let mut models: Vec<A> = vec![];
    for model in loader {
        models.push(A::from_json(model)?)
    }
    if models.len() > 0 {
        <A as ActiveModelTrait>::Entity::insert_many(models)
            .exec(db)
            .await?;
    }
    Ok(())
}

pub async fn check_init<C>(conn: &C) -> bluesnow_result::Result<bool>
where
    C: ConnectionTrait,
{
    match get_root_user(conn).await {
        Ok(_) => return Ok(true),
        Err(e) => {
            return match e {
                Error::NotFound(_) => Ok(false),
                _ => Err(e),
            }
        }
    }
}
pub async fn auto_migration<C>(conn: &C, param: &DbInitParam) -> bluesnow_result::Result<()>
where
    C: ConnectionTrait,
    for<'a> &'a C: IntoSchemaManagerConnection<'a>,
{
    Migrator::up(conn, None).await?;
    if !check_init(conn).await? {
        init_data(conn, param).await?;
    }
    Ok(())
}

async fn get_root_user<C>(conn: &C) -> bluesnow_result::Result<entity::user::Model>
where
    C: ConnectionTrait,
{
    let name = &Config::get()?.server.root_user;
    repo::user::get_by_username(conn, name).await
}
pub async fn setup_super_user<C>(conn: &C, param: &DbInitParam) -> bluesnow_result::Result<()>
where
    C: ConnectionTrait,
{
    let password = utils::pwd::hash(&param.root_password).await?;
    let root = match get_root_user(conn).await {
        Ok(model) => {
            let mut model = model.into_active_model();
            model.password = Set(password);
            model
        }
        Err(e) => match e {
            Error::NotFound(_) => entity::user::ActiveModel {
                username: Set(param.root_user.clone()),
                password: Set(password),
                email: Set("xxxxxx@xx.xx".to_string()),
                is_super_user: Set(true),
                ..Default::default()
            },
            _ => return Err(e),
        },
    };
    root.save(conn).await?;
    Ok(())
}
async fn init_data<C>(conn: &C, param: &DbInitParam) -> bluesnow_result::Result<()>
where
    C: ConnectionTrait,
{
    let roles = include_str!("../../data/role.json");
    seed::<entity::role::ActiveModel, C>(conn, roles).await?;
    // 初始化用户
    setup_super_user(conn, param).await?;
    let users = include_str!("../../data/user.json");
    seed::<entity::user::ActiveModel, C>(conn, users).await?;
    // 初始化api
    let apis = include_str!("../../data/api.json");
    seed::<entity::api::ActiveModel, C>(conn, apis).await?;
    let menus = include_str!("../../data/menu.json");
    seed::<entity::api::ActiveModel, C>(conn, menus).await?;
    let role_mtm_api = include_str!("../../data/role_mtm_api.json");
    seed::<entity::role_mtm_api::ActiveModel, C>(conn, role_mtm_api).await?;
    let role_mtm_menu = include_str!("../../data/role_mtm_menu.json");
    seed::<entity::role_mtm_menu::ActiveModel, C>(conn, role_mtm_menu).await?;
    let user_mtm_role = include_str!("../../data/user_mtm_role.json");
    seed::<entity::user_mtm_role::ActiveModel, C>(conn, user_mtm_role).await?;
    Ok(())
}

pub async fn refresh<C>(conn: &C, param: &DbInitParam) -> bluesnow_result::Result<()>
where
    C: ConnectionTrait,
    for<'a> &'a C: IntoSchemaManagerConnection<'a>,
{
    Migrator::refresh(conn).await?;
    init_data(conn, param).await?;
    Ok(())
}
