use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Article {
    Table,
    Id,
    UserId,
    Title,
    Desc,
    CategoryId,
    Hero,
    Cover,
    Content,
    UpdatedAt,
    CreatedAt,
}
