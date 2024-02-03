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

pub async fn tracklists_overview_handler() -> impl IntoResponse {
    println!("->> {:<12} - tracklists_overview_handler", "HANDLER");
    TracklistsOverviewTemplate
}

pub fn routes() -> Router<SqlitePool> {
    Router::new()
        .route(
            "/overview",
            get(routes::tracklists::tracklists_overview_handler),
        )
        .route("/artists", get(routes::tracklists::artists::get_artists))
        .route(
            "/artists/create",
            get(routes::tracklists::artists::create_artist)
                .post(routes::tracklists::artists::post_artist),
        )
        .route(
            "/artists/:id",
            get(routes::tracklists::artists::artist_detail_handler),
        )
        .route(
            "/mix-series",
            get(routes::tracklists::mix_series::mix_series_handler),
        )
        .route("/songs", get(routes::tracklists::songs::songs_handler))
        .route("/docs", get(routes::tracklists::docs::docs_handler))
}
