use iden::m0_1::Api::Icon;
use iden::m0_1::*;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{
    boolean, integer, integer_null, pk_auto, string, string_null, string_uniq,
    timestamp_with_time_zone, tiny_integer, uuid,
};
use std::string;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // 创建Api表
        manager
            .create_table(
                Table::create()
                    .table(Api::Table)
                    .col(pk_auto(Api::Id))
                    .col(string_uniq(Api::Name))
                    .col(string(Api::Path))
                    .col(string(Api::Method))
                    .col(string_null(Api::Code).unique_key())
                    .col(string_null(Icon))
                    .col(string_null(Api::Display))
                    .col(
                        timestamp_with_time_zone(User::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Role::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("api table created");
        // 创建文章表
        manager
            .create_table(
                Table::create()
                    .table(Article::Table)
                    .col(pk_auto(Article::Id))
                    .col(integer(Article::UserId))
                    .col(string_uniq(Article::Title))
                    .col(string(Article::Desc))
                    .col(string_null(Article::Hero))
                    .col(string_null(Article::Cover))
                    .col(string_null(Article::CategoryId))
                    .col(string(Article::Content))
                    .col(
                        timestamp_with_time_zone(Article::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Article::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("article table created");
        // 创建分类表
        manager
            .create_table(
                Table::create()
                    .table(Category::Table)
                    .col(pk_auto(Category::Id))
                    .col(integer_null(Category::PId))
                    .col(string_uniq(Category::Name))
                    .col(string(Category::Desc))
                    .col(
                        timestamp_with_time_zone(Category::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Category::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("category table created");
        // 创建菜单表
        manager
            .create_table(
                Table::create()
                    .table(Menu::Table)
                    .col(pk_auto(Menu::Id))
                    .col(string_null(Menu::Path).unique_key())
                    .col(string_uniq(Menu::Name))
                    .col(string_null(Menu::Component))
                    .col(boolean(Menu::IsVisible))
                    .col(integer(Menu::Status))
                    .col(boolean(Menu::KeepAlive))
                    .col(integer(Menu::Order))
                    .col(integer_null(Menu::PId))
                    .col(
                        timestamp_with_time_zone(Category::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Category::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("menu table created");
        // 创建角色表
        manager
            .create_table(
                Table::create()
                    .table(Role::Table)
                    .col(pk_auto(Role::Id))
                    .col(string_uniq(Role::Name))
                    .col(boolean(Role::IsBase).default(false))
                    .col(
                        timestamp_with_time_zone(Role::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Role::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("role table created");
        // 创建角色和api关联表
        manager
            .create_table(
                Table::create()
                    .table(RoleMtmApi::Table)
                    .col(integer(RoleMtmApi::RoleId))
                    .col(integer(RoleMtmApi::ApiId))
                    .primary_key(
                        Index::create()
                            .primary()
                            .col(RoleMtmApi::ApiId)
                            .col(RoleMtmApi::RoleId),
                    )
                    .to_owned(),
            )
            .await?;
        println!("role_mtm_api table created");
        // 创建角色和菜单关联表
        manager
            .create_table(
                Table::create()
                    .table(RoleMtmMenu::Table)
                    .col(integer(RoleMtmMenu::RoleId))
                    .col(integer(RoleMtmMenu::MenuId))
                    .primary_key(
                        Index::create()
                            .primary()
                            .col(RoleMtmMenu::RoleId)
                            .col(RoleMtmMenu::MenuId),
                    )
                    .to_owned(),
            )
            .await?;
        println!("role_mtm_menu table created");
        // 创建用户表
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .col(pk_auto(User::Id))
                    .col(string_uniq(User::Username))
                    .col(string(User::Password))
                    .col(string_uniq(User::Email))
                    .col(boolean(User::IsSuperUser).default(false))
                    .col(integer_null(User::ActiveRoleId))
                    .col(
                        timestamp_with_time_zone(User::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp_with_time_zone(Role::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        println!("user table created");
        // 创建用户和角色关联表
        manager
            .create_table(
                Table::create()
                    .table(UserMtmRole::Table)
                    .col(integer(UserMtmRole::UserId))
                    .col(integer(UserMtmRole::RoleId))
                    .primary_key(
                        Index::create()
                            .primary()
                            .col(UserMtmRole::UserId)
                            .col(UserMtmRole::RoleId),
                    )
                    .to_owned(),
            )
            .await?;
        println!("user_mtm_role table created");
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Api::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Article::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Category::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Menu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Role::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RoleMtmApi::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(RoleMtmMenu::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(UserMtmRole::Table).to_owned())
            .await?;
        Ok(())
    }
}
