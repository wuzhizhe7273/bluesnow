use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum Category {
    Table,
    Id,
    PId,
    Name,
    Desc,
    CreatedAt,
    UpdatedAt,
}
