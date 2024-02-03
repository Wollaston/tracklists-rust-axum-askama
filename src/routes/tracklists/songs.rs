use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/tracklists/songs.html")]
pub struct SongsTemplate;

pub async fn songs_handler() -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");
    SongsTemplate
}
