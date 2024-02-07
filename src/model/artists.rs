use axum::extract::Path;
use axum::Form;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use crate::ctx::Ctx;
use crate::Result;

use super::ModelManager;

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Artist {
    pub uuid: Uuid,
    pub creator_uuid: Uuid,
    pub artist_name: String,
    pub real_name: Option<String>,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistForCreate {
    pub artist_name: String,
    pub real_name: Option<String>,
}

pub struct ArtistsBmc;

impl ArtistsBmc {
    pub async fn get_artist(
        _ctx: Ctx,
        mm: &ModelManager,
        Path(uuid): Path<Uuid>,
    ) -> Result<Artist> {
        debug!("{:<12} - get_artist", "GET");

        let artist = sqlx::query_as::<_, Artist>(
            "
    SELECT * FROM artists WHERE uuid = $1
    ",
        )
        .bind(uuid)
        .fetch_one(mm.pool())
        .await
        .unwrap();

        Ok(artist)
    }

    pub async fn get_artists(_ctx: Ctx, mm: &ModelManager) -> Result<Vec<Artist>> {
        debug!("{:<12} - get_artists", "GET");

        let artists = sqlx::query_as::<_, Artist>(
            "
    SELECT * FROM artists
    ",
        )
        .fetch_all(mm.pool())
        .await
        .unwrap();

        Ok(artists)
    }

    pub async fn create_artist(
        ctx: Ctx,
        mm: &ModelManager,
        Form(input): Form<ArtistForCreate>,
    ) -> Result<Artist> {
        debug!("{:<12} - post_artist", "POST");

        let artist = Artist {
            uuid: Uuid::new_v4(),
            creator_uuid: ctx.user_id(),
            artist_name: input.artist_name,
            real_name: input.real_name,
            created_date: Utc::now(),
        };

        let returned_artist = sqlx::query_as::<_, Artist>(
            "
            INSERT INTO artists (uuid, creator_uuid, artist_name, real_name, created_date)
            values($1, $2, $3, $4, $5)
            RETURNING *
            ",
        )
        .bind(artist.uuid)
        .bind(artist.creator_uuid)
        .bind(artist.artist_name)
        .bind(artist.real_name.unwrap())
        .bind(artist.created_date)
        .fetch_one(mm.pool())
        .await
        .unwrap();

        Ok(returned_artist)
    }
}

//  endregion:  --- artist
