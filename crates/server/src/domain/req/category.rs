use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct CreateCategoryRequest {
    pub p_id: Option<i32>,
    pub name: String,
    pub desc: String,
}

#[derive(Deserialize, Clone)]
pub struct UpdateCategoryRequest {
    pub p_id: Option<i32>,
    pub name: String,
    pub desc: String,
}
