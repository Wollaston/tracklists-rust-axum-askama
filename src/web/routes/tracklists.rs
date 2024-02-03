use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};

use crate::{web, AppState};

pub mod artists;
pub mod docs;
pub mod mix_series;
pub mod songs;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/tracklists", sub_routes())
}

fn sub_routes() -> Router<AppState> {
    Router::new()
        .route(
            "/overview",
            get(web::routes::tracklists::tracklists_overview_handler),
        )
        .route(
            "/artists",
            get(web::routes::tracklists::artists::get_artists),
        )
        .route(
            "/artists/create",
            get(web::routes::tracklists::artists::create_artist)
                .post(web::routes::tracklists::artists::post_artist),
        )
        .route(
            "/artists/:id",
            get(web::routes::tracklists::artists::artist_detail_handler),
        )
        .route(
            "/mix-series",
            get(web::routes::tracklists::mix_series::mix_series_handler),
        )
        .route("/songs", get(web::routes::tracklists::songs::songs_handler))
        .route("/docs", get(web::routes::tracklists::docs::docs_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/overview.html")]
pub struct TracklistsOverviewTemplate;

pub async fn tracklists_overview_handler() -> impl IntoResponse {
    println!("->> {:<12} - tracklists_overview_handler", "HANDLER");
    TracklistsOverviewTemplate
}
