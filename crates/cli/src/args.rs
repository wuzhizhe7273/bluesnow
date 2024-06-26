use clap::Args;
use server::{DbInitParam, Environment, StartParam};
use std::path::PathBuf;

/// server start arguments
#[derive(Args)]
pub struct StartArgs {
    /// 服务器运行环境,可选:"dev","prod","test"
    #[arg(env = "BLUESNOW_ENV", long, short)]
    pub env: Environment,
    /// 配置文件夹路径
    #[arg(id = "config-path", long, short, env = "BLUESNOW_CONFIG_PATH")]
    pub config_path: PathBuf,
    pub addr: Option<String>,
    pub port: Option<u16>,
}

impl Into<StartParam> for StartArgs {
    fn into(self) -> StartParam {
        StartParam {
            env: self.env,
            config_path: self.config_path,
            addr: self.addr,
            port: self.port,
        }
    }
}

#[derive(Debug,Args)]
pub struct DbRefreshArgs{
    #[arg(id = "database-url",short,long,env="BLUESNOW_DB_URL")]
    pub database_url:String,
    #[arg(id="root-user",long,env="BLUESNOW_SERVER_ROOT_USER")]
    pub root_user:String,
    #[arg(id="root-password",long,env="BLUESNOW_SERVER_ROOT_PASSWORD")]
    pub root_password:String
}

impl Into<DbInitParam> for  DbRefreshArgs {
    fn into(self) -> DbInitParam {
        DbInitParam{
            root_user:self.root_user,
            root_password:self.root_password
        }
    }
}