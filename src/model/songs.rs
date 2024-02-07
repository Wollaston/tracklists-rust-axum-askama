use axum::extract::Path;
use axum::Form;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tracing::debug;
use uuid::Uuid;

use crate::Result;
use crate::{ctx::Ctx, model::ModelManager};

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

pub struct SongsBmc;

impl SongsBmc {
    pub async fn get_song(_ctx: Ctx, mm: &ModelManager, Path(uuid): Path<Uuid>) -> Result<Song> {
        debug!("{:<12} - get_song", "GET");

        let song = sqlx::query_as::<_, Song>(
            "
        SELECT * FROM songs WHERE uuid = $1
        ",
        )
        .bind(uuid)
        .fetch_one(mm.pool())
        .await
        .unwrap();

        Ok(song)
    }

    pub async fn get_songs(_ctx: Ctx, mm: &ModelManager) -> Result<Vec<Song>> {
        debug!("{:<12} - get_songs", "GET");
        let songs = sqlx::query_as::<_, Song>(
            "
    SELECT * FROM songs 
    ",
        )
        .fetch_all(mm.pool())
        .await
        .unwrap();

        Ok(songs)
    }

    pub async fn create_song(
        ctx: Ctx,
        mm: &ModelManager,
        Form(input): Form<SongForCreate>,
    ) -> Result<Song> {
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
        .fetch_one(mm.pool())
        .await
        .unwrap();

        Ok(returned_song)
    }
}
