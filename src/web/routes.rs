use axum::Router;

use crate::AppState;

pub mod about;
pub mod home;
pub mod not_found;
pub mod tracklists;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(home::routes())
        .merge(about::routes())
        .merge(tracklists::routes())
}
