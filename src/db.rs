use std::time::Duration;

use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

pub async fn db() -> Result<SqlitePool, sqlx::Error> {
    SqlitePoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect("sqlite:tracklists.db")
        .await
}

pub fn map_db_err(err: sqlx::Error) -> (axum::http::StatusCode, String) {
    tracing::error!("{}", err);
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        err.to_string(),
    )
}
