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

pub fn api_routes() -> Router<AppState> {
    Router::new().nest("/tracklists", api_sub_routes())
}

fn sub_routes() -> Router<AppState> {
    Router::new()
        .merge(overview::routes())
        .merge(artists::routes())
        .merge(mix_series::routes())
        .merge(songs::routes())
        .merge(docs::routes())
}

fn api_sub_routes() -> Router<AppState> {
    Router::new()
        .merge(artists::api_routes())
        .merge(songs::api_routes())
}
