use crate::Result;
use sqlx::SqlitePool;

pub mod artists;
mod error;
pub mod songs;
mod store;

// region: --- Model

/// The ModelManager holds the state of the application
/// e.g. DB Pool
/// There is one ModelManager and multiple Model Controllers, per entity
#[derive(Debug, Clone)]
pub struct ModelManager {
    pub pool: SqlitePool,
}

impl ModelManager {
    pub async fn new() -> Result<ModelManager> {
        let pool = store::db::db().await.unwrap();
        Ok(ModelManager { pool })
    }

    /// Returns a reference to the sqlx sqlite pool
    /// only for the model layer
    pub(in crate::model) fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

// endregion: --- model
