use std::fmt::{Display, Formatter};
use strum::EnumString;

#[derive(Debug, Clone)]
pub struct Resource {
    pub details: Vec<(String, String)>,
    pub resource_type: ResourceType,
}

impl Display for Resource {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.resource_type.fmt(f)
    }
}
#[derive(Debug, EnumString, strum::Display, Clone, Copy)]
pub enum ResourceType {
    #[strum(serialize = "USER")]
    User,
    #[strum(serialize = "ARTICLE")]
    Article,
    #[strum(serialize = "CATEGORY")]
    Category,
}
pub trait AppEntity {
    const RESOURCE: ResourceType;
}

impl AppEntity for entity::user::Model {
    const RESOURCE: ResourceType = ResourceType::User;
}

impl AppEntity for entity::article::Model {
    const RESOURCE: ResourceType = ResourceType::Article;
}

impl AppEntity for entity::category::Model {
    const RESOURCE: ResourceType = ResourceType::Category;
}
