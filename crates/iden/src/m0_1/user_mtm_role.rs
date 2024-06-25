use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum UserMtmRole {
    Table,
    UserId,
    RoleId,
}
