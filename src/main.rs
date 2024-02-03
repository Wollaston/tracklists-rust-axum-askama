use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub mod db;
pub mod error;
pub mod routes;

pub use self::error::{Error, Result};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn home_handler() -> impl IntoResponse {
    println!("->> {:<12} - home_handler", "HANDLER");
    HomeTemplate
}

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate;

async fn not_found_handler() -> impl IntoResponse {
    println!("->> {:<12} - not_found_handler", "HANDLER");
    NotFoundTemplate
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let pool = db::db().await.expect("Could not connect to sqlite DB.");

    let app = Router::new()
        .route("/", get(home_handler))
        .route("/about", get(routes::about::about_handler))
        .nest("/tracklists", routes::tracklists::tracklists_routes())
        .nest_service("/assets", ServeDir::new("public/assets/"))
        .nest_service("/css", ServeDir::new("style/"))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .route_service("/htmx.js", ServeFile::new("public/scripts/htmx.min.js"))
        .fallback(not_found_handler)
        .layer(TraceLayer::new_for_http())
        .with_state(pool);

    let port = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    println!("->> Listening on {port}\n");
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
