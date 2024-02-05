use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tracing::debug;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/overview", get(tracklists_overview_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/overview.html")]
struct TracklistsOverviewTemplate;

async fn tracklists_overview_handler() -> impl IntoResponse {
    debug!("{:<12} - tracklists_overview_handler", "HANDLER");
    TracklistsOverviewTemplate
}
