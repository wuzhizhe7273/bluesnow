use crate::config::Config;
use crate::context::Context;
use crate::environment::Environment;
use crate::routes::add_routes;
use crate::{DbInitParam, utils};
use axum::routing::get;
use axum::Router;
use bluesnow_result::Result;
use std::path::PathBuf;
use sea_orm::TransactionTrait;

pub struct BootResult {
    context: Context,
    router: Router,
}
#[derive(Clone)]
pub struct ServeParam {
    pub config_path: PathBuf,
    pub env: Environment,
    pub binding: Option<String>,
    pub port: Option<u16>,
}

/// 运行应用
pub async fn start(boot: BootResult) -> Result<()> {
    let BootResult { router, context } = boot;
    // 数据库自动迁移
    if Config::get()?.db.auto_migration {
        let root_password=Config::get()?.server.root_password.clone();
        let root_user=Config::get()?.server.root_user.clone();
        let param=DbInitParam{root_password,root_user};
        let tx=context.db.begin().await?;
        utils::db::auto_migration(&tx,&param).await?;
        tx.commit().await?;
        println!("commit");
    }
    // 启动服务器
    let addr = Config::get()?.server.get_http_addr();
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, router).await?;
    Ok(())
}

/// 创建应用
pub async fn create_app(config: &Config) -> Result<BootResult> {
    let context = create_context(config).await?;
    run_app(context).await
}

async fn create_context(config: &Config) -> Result<Context> {
    Context::new(config).await
}

pub async fn run_app(ctx: Context) -> Result<BootResult> {
    let router = Router::new().route("/_ping", get(|| async { "server is running" }));
    let router = add_routes(router, ctx.clone())?;
    let router = router.with_state(ctx.clone());
    Ok(BootResult {
        router,
        context: ctx,
    })
}
