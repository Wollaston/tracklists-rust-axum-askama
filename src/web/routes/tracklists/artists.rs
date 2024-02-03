use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    Form,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

use crate::AppState;

#[derive(Template)]
#[template(path = "routes/tracklists/artists.html")]
pub struct ArtistsTemplate {
    pub artists: Vec<Artist>,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artist_card.html")]
pub struct ArtistCardTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artist.html")]
pub struct ArtistTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artist_detail.html")]
pub struct ArtistDetailTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/create_artist.html")]
pub struct CreateArtistTemplate;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Artist {
    pub artist_id: i64,
    pub artist_name: String,
    pub real_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateArtist {
    pub artist_name: String,
    pub real_name: Option<String>,
}

pub async fn get_artist(State(state): State<AppState>, Path(id): Path<i64>) -> Artist {
    println!("->> {:<12} - get_artist", "GET");
    sqlx::query_as::<_, Artist>(
        "
    SELECT * FROM artists WHERE artist_id = $1
    ",
    )
    .bind(id)
    .fetch_one(&state.mc.pool)
    .await
    .unwrap()
}

pub async fn artist_detail_handler(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> impl IntoResponse {
    println!("->> {:<12} - artist_detail_handler", "HANDLER");
    let artist = get_artist(axum::extract::State(state), axum::extract::Path(id)).await;

    ArtistDetailTemplate { artist }
}

pub async fn post_artist(
    State(state): State<AppState>,
    Form(input): Form<CreateArtist>,
) -> impl IntoResponse {
    println!("->> {:<12} - post_artist", "POST");
    let id = sqlx::query(
        "
    INSERT INTO artists (artist_name, real_name)
    values($1, $2)
RETURNING *
    ",
    )
    .bind(input.artist_name)
    .bind(input.real_name.unwrap())
    .execute(&state.mc.pool)
    .await
    .unwrap()
    .last_insert_rowid();

    let artist = get_artist(axum::extract::State(state), axum::extract::Path(id)).await;

    ArtistTemplate { artist }
}

pub async fn create_artist() -> impl IntoResponse {
    println!("->> {:<12} - creat_artist_handler", "HANDLER");
    CreateArtistTemplate
}

pub async fn get_artists(State(state): State<AppState>) -> impl IntoResponse {
    println!("->> {:<12} - get_artists", "GET");
    let artists = sqlx::query_as::<_, Artist>(
        "
    SELECT * FROM artists
    ",
    )
    .fetch_all(&state.mc.pool)
    .await
    .unwrap();

    ArtistsTemplate { artists }
}
