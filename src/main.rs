use askama::Template;
use askama_axum::IntoResponse;
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

pub mod db;
pub mod routes;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate;

async fn home() -> impl IntoResponse {
    HomeTemplate
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    let pool = db::db().await.expect("Could not connect to sqlite DB.");

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(routes::about::about))
        .route("/tracklists", get(routes::tracklists::tracklists))
        .nest("/tracklists", routes::tracklists::tracklists_routes())
        .nest_service("/assets", ServeDir::new("public/assets/"))
        .nest_service("/css", ServeDir::new("style/"))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .route_service("/htmx.js", ServeFile::new("public/scripts/htmx.min.js"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
