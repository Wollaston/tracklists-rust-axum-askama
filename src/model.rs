use crate::Result;
use axum::{extract::Path, Form};
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
    pub async fn get_song(&self, Path(uuid): Path<Uuid>) -> Result<Song> {
        println!("->> {:<12} - get_song", "GET");

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

//  region:     --- artist

#[derive(Debug, sqlx::FromRow, Serialize, Deserialize)]
pub struct Artist {
    pub artist_id: i64,
    pub artist_name: String,
    pub real_name: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtistForCreate {
    pub artist_name: String,
    pub real_name: Option<String>,
}

impl ModelController {
    pub async fn get_artist(&self, Path(id): Path<i64>) -> Result<Artist> {
        println!("->> {:<12} - get_artist", "GET");

        let artist = sqlx::query_as::<_, Artist>(
            "
    SELECT * FROM artists WHERE artist_id = $1
    ",
        )
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(artist)
    }

    pub async fn get_artists(&self) -> Result<Vec<Artist>> {
        println!("->> {:<12} - get_artists", "GET");

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

    pub async fn create_artist(&self, Form(input): Form<ArtistForCreate>) -> Result<Artist> {
        println!("->> {:<12} - post_artist", "POST");
        let artist = sqlx::query_as::<_, Artist>(
            "
            INSERT INTO artists (artist_name, real_name)
            values($1, $2)
            RETURNING *
            ",
        )
        .bind(input.artist_name)
        .bind(input.real_name.unwrap())
        .fetch_one(&self.pool)
        .await
        .unwrap();

        Ok(artist)
    }
}

//  endregion:  --- artist
