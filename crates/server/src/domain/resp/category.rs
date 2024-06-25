use crate::utils::tree::{Tree, TreeNode};
use chrono::{DateTime, FixedOffset};
use entity::category::Model;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct CreateCategoryResponse {
    id: i32,
}

impl CreateCategoryResponse {
    pub fn new(id: i32) -> Self {
        Self { id }
    }
}

pub struct CategoryListResponseNode {
    pub id: i32,
    pub p_id: Option<i32>,
    pub name: String,
    pub desc: String,
    pub created_at: DateTime<FixedOffset>,
    pub updated_at: DateTime<FixedOffset>,
}

impl TreeNode for CategoryListResponseNode {
    type Id = i32;
    fn key(&self) -> Self::Id {
        self.id
    }
    fn parent_key(&self) -> Option<Self::Id> {
        self.p_id
    }
}

impl From<entity::category::Model> for CategoryListResponseNode {
    fn from(value: Model) -> Self {
        CategoryListResponseNode {
            id: value.id,
            p_id: value.p_id,
            name: value.name,
            desc: value.desc,
            created_at: value.created_at,
            updated_at: value.updated_at,
        }
    }
}

impl From<Vec<entity::category::Model>> for Tree<CategoryListResponseNode> {
    fn from(value: Vec<Model>) -> Self {
        let nodes: Vec<CategoryListResponseNode> = value.into_iter().map(|v| v.into()).collect();
        nodes.into()
    }
}
