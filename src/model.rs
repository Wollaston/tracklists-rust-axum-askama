use crate::{ctx::Ctx, Result};
use axum::{extract::Path, Form};
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
use tracing::debug;
use uuid::Uuid;

// region: --- Model

#[derive(Debug, Clone)]
pub struct ModelController {
    pub pool: SqlitePool,
}

impl ModelController {
    pub async fn new(pool: SqlitePool) -> Result<ModelController> {
        Ok(ModelController { pool })
    }
}

// endregion: --- model

// region: --- song

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Song {
    pub uuid: Uuid,
    pub creator_uuid: Uuid,
    pub title: String,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SongForCreate {
    pub title: String,
}

impl ModelController {
    pub async fn get_song(&self, _ctx: Ctx, Path(uuid): Path<Uuid>) -> Result<Song> {
        debug!("{:<12} - get_song", "GET");

        let song = sqlx::query_as::<_, Song>(
            "
        SELECT * FROM songs WHERE uuid = $1
        ",
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(song)
    }

    pub async fn get_songs(&self, _ctx: Ctx) -> Result<Vec<Song>> {
        debug!("{:<12} - get_songs", "GET");
        let songs = sqlx::query_as::<_, Song>(
            "
    SELECT * FROM songs 
    ",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(songs)
    }

    pub async fn create_song(&self, ctx: Ctx, Form(input): Form<SongForCreate>) -> Result<Song> {
        debug!("{:<12} - create_song", "POST");

        let song = Song {
            uuid: Uuid::new_v4(),
            creator_uuid: ctx.user_id(),
            title: input.title,
            created_date: Utc::now(),
        };

        let returned_song = sqlx::query_as::<_, Song>(
            "
    INSERT INTO songs (uuid, creator_uuid, title, created_date)
        values($1, $2, $3, $4)
        RETURNING *
    ",
        )
        .bind(song.uuid)
        .bind(song.creator_uuid)
        .bind(song.title.clone())
        .bind(song.created_date)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(returned_song)
    }
}

// endregion: --song

//  region:     --- artist

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

impl ModelController {
    pub async fn get_artist(&self, _ctx: Ctx, Path(uuid): Path<Uuid>) -> Result<Artist> {
        debug!("{:<12} - get_artist", "GET");

        let artist = sqlx::query_as::<_, Artist>(
            "
    SELECT * FROM artists WHERE uuid = $1
    ",
        )
        .bind(uuid)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(artist)
    }

    pub async fn get_artists(&self, _ctx: Ctx) -> Result<Vec<Artist>> {
        debug!("{:<12} - get_artists", "GET");

        let artists = sqlx::query_as::<_, Artist>(
            "
    SELECT * FROM artists
    ",
        )
        .fetch_all(&self.pool)
        .await
        .unwrap();

        Ok(artists)
    }

    pub async fn create_artist(
        &self,
        ctx: Ctx,
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
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(returned_artist)
    }
}

//  endregion:  --- artist
