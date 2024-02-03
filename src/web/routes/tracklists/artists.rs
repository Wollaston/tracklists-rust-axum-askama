use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::get,
    Form, Router,
};

use crate::model::Artist;
use crate::{model::ArtistForCreate, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/artists", get(artists_handler))
        .route(
            "/artists/create",
            get(create_artist_handler).post(create_artist_post),
        )
        .route("/artists/:id", get(artist_detail_handler))
}

#[derive(Template)]
#[template(path = "routes/tracklists/artists/artists.html")]
pub struct ArtistsTemplate {
    pub artists: Vec<Artist>,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artists/artist_card.html")]
pub struct ArtistCardTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artists/artist.html")]
pub struct ArtistTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artists/artist_detail.html")]
pub struct ArtistDetailTemplate {
    artist: Artist,
}

#[derive(Template)]
#[template(path = "routes/tracklists/artists/create_artist.html")]
pub struct CreateArtistTemplate;

pub async fn artist_detail_handler(
    State(state): State<AppState>,
    id: Path<i64>,
) -> impl IntoResponse {
    println!("->> {:<12} - artist_detail_handler", "HANDLER");
    let artist = state.mc.get_artist(id).await.unwrap();

    ArtistDetailTemplate { artist }
}

pub async fn create_artist_post(
    State(state): State<AppState>,
    input: Form<ArtistForCreate>,
) -> impl IntoResponse {
    let artist = state.mc.create_artist(input).await.unwrap();

    ArtistTemplate { artist }
}

pub async fn create_artist_handler() -> impl IntoResponse {
    println!("->> {:<12} - creat_artist_handler", "HANDLER");
    CreateArtistTemplate
}

pub async fn artists_handler(State(state): State<AppState>) -> impl IntoResponse {
    let artists = state.mc.get_artists().await.unwrap();

    ArtistsTemplate { artists }
}
