use sea_orm_migration::prelude::*;
#[derive(DeriveIden)]
pub enum Menu {
    Table,
    Id,        //菜单Id
    Path,      // 路由地址
    Name,      //菜单名
    Component, //组件路径
    IsVisible, // 显示状态
    Status,    // 状态
    KeepAlive, //前端KeepAlive
    Order,     // 排序
    PId,       //父菜单Id

    UpdatedAt,
    CreatedAt,
}
