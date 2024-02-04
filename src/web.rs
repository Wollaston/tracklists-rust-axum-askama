use axum::Router;

use crate::AppState;

pub mod api;
pub mod middleware;
pub mod routes;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn routes() -> Router<AppState> {
    Router::new().merge(api::routes()).merge(routes::routes())
}
