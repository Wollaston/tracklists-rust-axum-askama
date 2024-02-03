use askama::Template;
use askama_axum::IntoResponse;
use axum::{extract::State, routing::get, Form, Router};

use crate::{
    model::{Song, SongForCreate},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/songs", get(songs_handler))
        .route("/songs/create", get(create_songs_handler).post(create_song))
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs/songs.html")]
pub struct SongsTemplate {
    pub songs: Vec<Song>,
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs/song_card.html")]
pub struct SongCardTemplate {
    song: Song,
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs/song.html")]
pub struct SongTemplate {
    song: Song,
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs/song_detail.html")]
pub struct SongDetailTemplate {
    song: Song,
}

#[derive(Template)]
#[template(path = "routes/tracklists/songs/create_song.html")]
pub struct CreateSongTemplate;

async fn songs_handler(State(state): State<AppState>) -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");

    let songs: Vec<Song> = state.mc.get_songs().await.unwrap();

    SongsTemplate { songs }
}

async fn create_songs_handler() -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");

    CreateSongTemplate
}

async fn create_song(
    State(state): State<AppState>,
    input: Form<SongForCreate>,
) -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");

    let song = state.mc.create_song(input).await.unwrap();

    SongTemplate { song }
}
