use axum::Router;

use crate::AppState;

pub mod login;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/api", login::routes())
}
