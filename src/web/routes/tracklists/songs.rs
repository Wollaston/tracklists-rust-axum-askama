use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};

use crate::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().route("/songs", get(songs_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs.html")]
struct SongsTemplate;

async fn songs_handler() -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");
    SongsTemplate
}
