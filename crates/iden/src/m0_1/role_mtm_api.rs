use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum RoleMtmApi {
    Table,
    RoleId,
    ApiId,
}
