use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};
use tracing::debug;

use crate::{
    ctx::Ctx,
    model::songs::{Song, SongForCreate, SongsBmc},
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
    debug!("{:<12} - songs_handler", "HANDLER");

    let songs: Vec<Song> = SongsBmc::get_songs(ctx, &state.mm).await.unwrap();

    SongsTemplate { songs }
}

async fn create_song_handler() -> impl IntoResponse {
    debug!("{:<12} - create_songs_handler", "HANDLER");

    CreateSongTemplate
}

async fn create_song_post(
    State(state): State<AppState>,
    ctx: Ctx,
    input: Form<SongForCreate>,
) -> impl IntoResponse {
    debug!("{:<12} - create_song_handler", "HANDLER");

    let song = SongsBmc::create_song(ctx, &state.mm, input).await.unwrap();

    SongTemplate { song }
}

pub async fn song_detail_handler(
    State(state): State<AppState>,
    ctx: Ctx,
    uuid: Path<uuid::Uuid>,
) -> impl IntoResponse {
    debug!("{:<12} - song_detail_handler", "HANDLER");
    let song = SongsBmc::get_song(ctx, &state.mm, uuid).await.unwrap();

    SongDetailTemplate { song }
}
