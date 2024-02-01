use askama::Template;
use askama_axum::IntoResponse;
use axum::extract::State;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "routes/tracklists/artists.html")]
pub struct ArtistsTemplate {
    pub artists: Vec<Artist>,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artist.html")]
pub struct ArtistTemplate {
    artist: Artist,
}

#[derive(sqlx::FromRow, Serialize, Deserialize)]
pub struct Artist {
    pub artist_id: i64,
    pub artist_name: String,
    pub real_name: Option<String>,
}

pub async fn artist(State(pool): State<SqlitePool>) -> impl IntoResponse {
    let artist = sqlx::query_as::<_, Artist>(
        "
    SELECT * FROM artists
    ",
    )
    .fetch_one(&pool)
    .await
    .unwrap();

    ArtistTemplate { artist }
}

pub async fn artists(State(pool): State<SqlitePool>) -> impl IntoResponse {
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
