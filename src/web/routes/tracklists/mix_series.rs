use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tracing::info;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/mix-series", get(mix_series_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/mix-series.html")]
struct MixSeriesTemplate;

async fn mix_series_handler() -> impl IntoResponse {
    info!("{:<12} - mix_series_handler", "HANDLER");
    MixSeriesTemplate
}
