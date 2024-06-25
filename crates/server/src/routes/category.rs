use crate::context::Context;
use crate::handler::category;
use axum::routing::post;
use axum::Router;

pub fn add_routes(router: Router<Context>, context: Context) -> Router<Context> {
    let router = router.route("/api/v1/category", post(category::create));
    router
}
