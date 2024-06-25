use crate::config::Config;
use crate::utils;
use bluesnow_result::Result;
use sea_orm::DbConn;

#[derive(Clone)]
pub struct Context {
    pub db: DbConn,
}

impl Context {
    pub async fn new(config: &Config) -> Result<Self> {
        let db = utils::db::connect(&config).await?;
        let context = Context { db };
        Ok(context)
    }
}
