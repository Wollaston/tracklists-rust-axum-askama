use axum::{middleware, Router};

use crate::AppState;

pub mod about;
pub mod home;
pub mod not_found;
pub mod tracklists;

pub fn routes() -> Router<AppState> {
    let tracklists_routes = tracklists::routes().route_layer(middleware::from_fn(
        crate::web::middleware::auth::mw_require_auth,
    ));

    Router::new()
        .merge(home::routes())
        .merge(about::routes())
        .merge(tracklists_routes)
}
