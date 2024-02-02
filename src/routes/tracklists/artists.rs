use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    Form,
};
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

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

pub async fn get_artist(State(pool): State<SqlitePool>, Path(id): Path<i64>) -> impl IntoResponse {
    let artist = sqlx::query_as::<_, Artist>(
        "
    SELECT * FROM artists WHERE artist_id = $1
    ",
    )
    .bind(id)
    .fetch_one(&pool)
    .await
    .unwrap();

    ArtistTemplate { artist }
}

pub async fn post_artist(
    State(pool): State<SqlitePool>,
    Form(input): Form<CreateArtist>,
) -> impl IntoResponse {
    let id = sqlx::query(
        "
    INSERT INTO artists (artist_name, real_name)
    values($1, $2)
RETURNING *
    ",
    )
    .bind(input.artist_name)
    .bind(input.real_name.unwrap())
    .execute(&pool)
    .await
    .unwrap()
    .last_insert_rowid();

    get_artist(axum::extract::State(pool), axum::extract::Path(id)).await
}

pub async fn create_artist() -> impl IntoResponse {
    CreateArtistTemplate
}

pub async fn get_artists(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let artists = sqlx::query_as::<_, Artist>(
        "
    SELECT * FROM artists
    ",
    )
    .fetch_all(&pool)
    .await
    .unwrap();

    ArtistsTemplate { artists }
}
