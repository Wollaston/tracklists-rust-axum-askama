use axum::Router;

use crate::AppState;

pub mod login;

pub fn routes() -> Router<AppState> {
    Router::new()
        .nest("/api", crate::web::routes::tracklists::api_routes())
        .route_layer(axum::middleware::from_fn(
            crate::web::middleware::auth::mw_require_auth,
        ))
}
