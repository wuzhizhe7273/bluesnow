use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Username,
    Password,
    Email,
    IsSuperUser,
    ActiveRoleId,
    CreatedAt,
    UpdatedAt,
}
