use crate::user::Entity;
use crate::user_mtm_role::Relation;
use crate::{api, role_mtm_api};
use sea_orm::{LinkDef, Linked, RelationTrait};

pub struct UserToApi;
impl Linked for UserToApi {
    type FromEntity = Entity;
    type ToEntity = api::Entity;
    fn link(&self) -> Vec<LinkDef> {
        vec![
            Relation::User.def().rev(),
            Relation::Role.def(),
            role_mtm_api::Relation::Api.def().rev(),
            role_mtm_api::Relation::Role.def(),
        ]
    }
}
