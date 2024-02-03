use crate::Result;
use axum::Form;
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;
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
    pub title: String,
    pub created_date: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SongForCreate {
    pub title: String,
}

impl ModelController {
    pub async fn get_songs(&self) -> Result<Vec<Song>> {
        println!("->> {:<12} - get_songs", "GET");
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

    pub async fn create_song(&self, Form(input): Form<SongForCreate>) -> Result<Song> {
        println!("->> {:<12} - create_song", "POST");

        let song = Song {
            uuid: Uuid::new_v4(),
            title: input.title,
            created_date: Utc::now(),
        };

        let returned_song = sqlx::query_as::<_, Song>(
            "
    INSERT INTO songs (uuid, title, created_date)
        values($1, $2, $3)
        RETURNING *
    ",
        )
        .bind(song.uuid)
        .bind(song.title.clone())
        .bind(song.created_date)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        print!("{:?}", returned_song);

        Ok(song)
    }
}

// endregion: --song
