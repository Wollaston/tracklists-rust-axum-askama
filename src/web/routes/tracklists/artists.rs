use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    routing::{get, post},
    Form, Router,
};
use tracing::debug;

use crate::{ctx::Ctx, model::Artist};
use crate::{model::ArtistForCreate, AppState};

pub fn routes() -> Router<AppState> {
    Router::new()
        .route("/artists", get(artists_handler))
        .route("/artists/:uuid", get(artist_detail_handler))
        .route("/artists/create", get(create_artist_handler))
}

pub fn api_routes() -> Router<AppState> {
    Router::new().route("/artists/create", post(create_artist_post))
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
    ctx: Ctx,
    uuid: Path<uuid::Uuid>,
) -> impl IntoResponse {
    debug!("{:<12} - artist_detail_handler", "HANDLER");
    let artist = state.mc.get_artist(ctx, uuid).await.unwrap();

    ArtistDetailTemplate { artist }
}

pub async fn create_artist_post(
    State(state): State<AppState>,
    ctx: Ctx,
    input: Form<ArtistForCreate>,
) -> impl IntoResponse {
    let artist = state.mc.create_artist(ctx, input).await.unwrap();

    ArtistTemplate { artist }
}

pub async fn create_artist_handler() -> impl IntoResponse {
    debug!("{:<12} - create_artist_handler", "HANDLER");
    CreateArtistTemplate
}

pub async fn artists_handler(ctx: Ctx, State(state): State<AppState>) -> impl IntoResponse {
    debug!("{:<12} - artists_handler", "HANDLER");
    let artists = state.mc.get_artists(ctx).await.unwrap();

    ArtistsTemplate { artists }
}
