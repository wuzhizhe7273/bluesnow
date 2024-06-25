//! `SeaORM` Entity, @generated by sea-orm-codegen 1.0.0-rc.6

use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "role_mtm_api"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel, Eq, Serialize, Deserialize)]
pub struct Model {
    pub role_id: i32,
    pub api_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    RoleId,
    ApiId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    ApiId,
    RoleId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);
    fn auto_increment() -> bool {
        false
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Role,
    Api,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::RoleId => ColumnType::Integer.def(),
            Self::ApiId => ColumnType::Integer.def(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Role => Entity::belongs_to(super::role::Entity)
                .from(Column::RoleId)
                .to(super::role::Column::Id)
                .into(),
            Self::Api => Entity::belongs_to(super::api::Entity)
                .from(Column::ApiId)
                .to(super::api::Column::Id)
                .into(),
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}