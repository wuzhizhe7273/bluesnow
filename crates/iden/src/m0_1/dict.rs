use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum Dict {
    Id, //主键
    Key,
}
