use axum::Router;

use crate::AppState;

pub mod login;

pub fn routes() -> Router<AppState> {
    Router::new().merge(login::routes())
}
