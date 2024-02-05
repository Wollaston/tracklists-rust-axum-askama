use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};

use crate::{
    ctx::Ctx,
    model::{Song, SongForCreate},
    AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/songs", get(songs_handler))
        .route("/songs/:uuid", get(song_detail_handler))
        .route("/songs/create", get(create_song_handler))
}

pub fn api_routes() -> Router<AppState> {
    Router::new().route("/songs/create", post(create_song_post))
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

async fn songs_handler(State(state): State<AppState>, ctx: Ctx) -> impl IntoResponse {
    println!("->> {:<12} - songs_handler", "HANDLER");

    let songs: Vec<Song> = state.mc.get_songs(ctx).await.unwrap();

    SongsTemplate { songs }
}

async fn create_song_handler() -> impl IntoResponse {
    println!("->> {:<12} - create_songs_handler", "HANDLER");

    CreateSongTemplate
}

async fn create_song_post(
    State(state): State<AppState>,
    ctx: Ctx,
    input: Form<SongForCreate>,
) -> impl IntoResponse {
    println!("->> {:<12} - create_song_handler", "HANDLER");

    let song = state.mc.create_song(ctx, input).await.unwrap();

    SongTemplate { song }
}

pub async fn song_detail_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    uuid: Path<uuid::Uuid>,
) -> impl IntoResponse {
    println!("->> {:<12} - song_detail_handler", "HANDLER");
    let song = state.mc.get_song(ctx, uuid).await.unwrap();

    SongDetailTemplate { song }
}
