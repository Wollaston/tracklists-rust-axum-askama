use axum::Router;
use sqlx::SqlitePool;

pub mod login;

pub fn routes() -> Router<SqlitePool> {
    Router::new().merge(login::routes())
}
