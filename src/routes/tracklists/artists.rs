use crate::db::map_db_err;
use askama::Template;
use axum::extract::State;
use sqlx::SqlitePool;

#[derive(Template)]
#[template(path = "routes/tracklists/artists.html")]
pub struct ArtistsTemplate {
    pub artists: Vec<ArtistTemplate>,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artist.html")]
pub struct ArtistTemplate {
    pub artist_id: i64,
    pub artist_name: String,
    pub real_name: Option<String>,
}

pub async fn artist(
    State(pool): State<SqlitePool>,
) -> Result<ArtistTemplate, (axum::http::StatusCode, String)> {
    sqlx::query_as!(
        ArtistTemplate,
        "
    SELECT * FROM artists
    "
    )
    .fetch_one(&pool)
    .await
    .map_err(map_db_err)
}

pub async fn artists(
    State(pool): State<SqlitePool>,
) -> Result<ArtistsTemplate, (axum::http::StatusCode, String)> {
    sqlx::query_as!(
        ArtistTemplate,
        "
    SELECT * FROM artists
    "
    )
    .fetch_all(&pool)
    .await
    .map(|artists| ArtistsTemplate { artists })
    .map_err(map_db_err)
}
