use axum::Router;
use sqlx::SqlitePool;

use crate::AppState;

pub mod login;
pub mod routes;

pub const AUTH_TOKEN: &str = "auth-token";

pub fn routes() -> Router<AppState> {
    Router::new().merge(login::routes())
}
