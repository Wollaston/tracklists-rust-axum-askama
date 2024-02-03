use axum::Router;

use crate::AppState;

pub mod about;
pub mod tracklists;

pub fn routes() -> Router<AppState> {
    Router::new()
        .merge(about::routes())
        .merge(tracklists::routes())
}
