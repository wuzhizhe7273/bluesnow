use crate::config::db::DatabaseConfig;
use crate::config::jwt::JwtConfig;
use crate::config::server::ServerConfig;
use crate::environment::Environment;
use crate::param::StartParam;
use bluesnow_result::Result;
use figment::providers::{Env, Format, Toml};
use serde::Deserialize;
use std::path::PathBuf;
use std::sync::OnceLock;

mod db;
mod jwt;
mod server;

static CONFIG: OnceLock<Config> = OnceLock::new();

#[derive(Debug, Deserialize)]
pub struct Config {
    pub env: Environment,
    pub server: ServerConfig,
    pub db: DatabaseConfig,
    pub jwt: JwtConfig,
}

impl Config {
    pub fn init(param: StartParam) -> Result<()> {
        let StartParam {
            env, config_path, ..
        } = &param;
        let mut config = Self::from_folder(config_path, env)?;
        config.merge_start_param(param);
        CONFIG
            .set(config)
            .map_err(|e| figment::Error::from(format!("initialize config:{:#?} failed", e)))?;
        Ok(())
    }
    fn merge_start_param(&mut self, param: StartParam) {
        let StartParam { port, addr, .. } = param;
        if let Some(port) = port {
            self.server.port = port
        }
        if let Some(addr) = addr {
            self.server.addr = addr
        }
    }
    pub fn get() -> Result<&'static Self> {
        let config = CONFIG
            .get()
            .ok_or(figment::Error::from("get config failed"))?;
        Ok(config)
    }
    fn from_folder(path: &PathBuf, environment: &Environment) -> Result<Self> {
        let figment = figment::Figment::new()
            .merge(Toml::file(path.join("base.toml")))
            .merge(Toml::file(path.join(format!("{},toml", environment))))
            .merge(Env::prefixed("BLUESNOW_"))
            .join(("env", environment));
        let config: Self = figment.extract()?;
        Ok(config)
    }
}
