use bluesnow_result::Result;
use clap::Args;
use server::{DbInitParam, Environment};
use std::path::PathBuf;
use sea_orm::TransactionTrait;
use crate::args::DbRefreshArgs;

#[derive(Debug, clap::Subcommand)]
pub enum DbCommands {
    Refresh {
        #[command(flatten)]
        arg:DbRefreshArgs
    },
}
impl DbCommands {
    pub async fn exec(self) -> Result<()> {
        match self {
            Self::Refresh { arg } => {
                let conn=sea_orm::Database::connect(arg.database_url.clone()).await?;
                let tx=conn.begin().await?;
                server::utils::db::refresh(&tx,&arg.into()).await?;
                tx.commit().await?;

            }
        }
        Ok(())
    }
}
