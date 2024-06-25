use sea_orm_migration::prelude::*;

#[derive(DeriveIden)]
pub enum Api {
    Table,
    Id,      //api id
    Name,    //api名
    Path,    //api路由
    Method,  //api 方法
    Code,    // 权限编码，供前端
    Icon,    // 如果有对应的按钮
    Display, // 前端显示文字
    CreatedAt,
    UpdatedAt,
}
