use axum::Router;
use sqlx::SqlitePool;

pub mod login;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn routes() -> Router<SqlitePool> {
    Router::new().merge(login::routes())
}
