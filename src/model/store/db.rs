use std::time::Duration;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

use crate::config;
use crate::model::store::error::Error;

pub async fn db() -> Result<SqlitePool, Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&config().DATABASE_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}
