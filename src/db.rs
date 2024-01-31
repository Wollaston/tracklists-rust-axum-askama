use std::time::Duration;

use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite};

pub async fn db() -> Result<Pool<Sqlite>, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("sqlite:tracklists.db")
        .await
}
