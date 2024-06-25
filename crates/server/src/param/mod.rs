use crate::environment::Environment;
use std::path::PathBuf;
pub struct StartParam {
    pub env: Environment,
    pub config_path: PathBuf,
    pub addr: Option<String>,
    pub port: Option<u16>,
}

pub struct DbInitParam{
    pub root_user:String,
    pub root_password:String,
}