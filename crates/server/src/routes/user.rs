use crate::context::Context;
use crate::handler::user;
use axum::routing::post;
use axum::Router;

pub fn add_routes(router: Router<Context>, context: Context) -> Router<Context> {
    let router = router
        .route("/api/v1/user/register", post(user::register))
        .route("/api/v1/user/login", post(user::login));
    router
}
