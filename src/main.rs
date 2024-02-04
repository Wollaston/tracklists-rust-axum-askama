use axum::{extract::FromRef, middleware, response::Response, Router};
use model::ModelController;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub mod ctx;
pub mod db;
pub mod error;
pub mod model;
pub mod web;

pub use self::error::{Error, Result};
#[derive(Clone, FromRef)]
pub struct AppState {
    pub mc: ModelController,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = db::db().await.expect("Could not connect to sqlite DB.");

    let app_state = AppState {
        mc: ModelController::new(pool).await.unwrap(),
    };

    let app = Router::new()
        .merge(web::routes())
        .nest_service("/assets", ServeDir::new("public/assets/"))
        .nest_service("/css", ServeDir::new("style/"))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .route_service("/htmx.js", ServeFile::new("public/scripts/htmx.min.js"))
        .fallback(web::routes::not_found::not_found_handler)
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let port = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    println!("->> Listening on {port}\n");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();

    res
}
