use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tracing::debug;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/docs", get(docs_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/docs.html")]
struct DocsTemplate;

async fn docs_handler() -> impl IntoResponse {
    debug!("{:<12} - docs_handler", "HANDLER");
    DocsTemplate
}
