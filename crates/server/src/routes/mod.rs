mod category;
mod user;

use crate::context::Context;
use crate::middleware::auth;
use axum::middleware::from_fn_with_state;
use axum::Router;
use bluesnow_result::Result;

pub fn add_routes(router: Router<Context>, context: Context) -> Result<Router<Context>> {
    let router = user::add_routes(router, context.clone());
    let router = router.layer(from_fn_with_state(context, auth));
    Ok(router)
}
