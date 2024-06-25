use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub addr: String,
    pub port: u16,
    pub root_password: String,
    pub root_user:String
}

impl ServerConfig {
    pub fn get_http_addr(&self) -> String {
        format!("{}:{}", self.addr, self.port)
    }
}
