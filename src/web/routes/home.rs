use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tracing::info;

use crate::AppState;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

pub fn routes() -> Router<AppState> {
    Router::new().route("/", get(home_handler))
}

async fn home_handler() -> impl IntoResponse {
    info!("{:<12} - home_handler", "HANDLER");
    HomeTemplate
}
