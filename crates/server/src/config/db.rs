use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    url: String,
    pub auto_migration: bool,
}
impl DatabaseConfig {
    pub fn get_url(&self) -> &String {
        &self.url
    }
}
