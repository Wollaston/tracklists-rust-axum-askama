use axum::Router;

use crate::AppState;

pub mod artists;
pub mod docs;
pub mod mix_series;
pub mod overview;
pub mod songs;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/tracklists", sub_routes())
}

fn sub_routes() -> Router<AppState> {
    Router::new()
        .merge(overview::routes())
        .merge(artists::routes())
        .merge(mix_series::routes())
        .merge(songs::routes())
        .merge(docs::routes())
}
