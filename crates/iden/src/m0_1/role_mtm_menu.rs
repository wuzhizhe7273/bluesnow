use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum RoleMtmMenu {
    Table,
    RoleId,
    MenuId,
}
