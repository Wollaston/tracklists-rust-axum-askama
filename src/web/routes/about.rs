use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tracing::info;

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/about", get(about_handler))
}

#[derive(Template)]
#[template(path = "routes/about.html")]
pub struct AboutTemplate;

pub async fn about_handler() -> impl IntoResponse {
    info!("{:<12} - about_handler", "HANDLER");
    AboutTemplate
}
