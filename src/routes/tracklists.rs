use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use sqlx::SqlitePool;

use crate::routes;

pub mod artists;
pub mod docs;
pub mod mix_series;
pub mod songs;

#[derive(Template)]
#[template(path = "routes/tracklists/overview.html")]
pub struct TracklistsOverviewTemplate;

pub async fn tracklists() -> impl IntoResponse {
    TracklistsOverviewTemplate
}

pub fn tracklists_routes() -> Router<SqlitePool> {
    Router::new()
        .route("/artists", get(routes::tracklists::artists::artists))
        .route(
            "/mix-series",
            get(routes::tracklists::mix_series::mix_series),
        )
        .route("/songs", get(routes::tracklists::songs::songs))
        .route("/docs", get(routes::tracklists::docs::docs))
}
