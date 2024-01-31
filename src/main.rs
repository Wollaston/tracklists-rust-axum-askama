use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

pub mod db;
pub mod routes;

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate {}

async fn home() -> HomeTemplate {
    HomeTemplate {}
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    tracing_subscriber::fmt::init();

    let pool = db::db().await.expect("Could not connect to sqlite DB.");

    let app = Router::new()
        .route("/", get(home))
        .route("/about", get(routes::about::about))
        .merge(tracklists(TracklistState {}))
        .nest_service("/assets", ServeDir::new("public/assets/"))
        .nest_service("/css", ServeDir::new("style/"))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"))
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

#[derive(Clone)]
struct TracklistState {}

fn tracklists<S>(state: TracklistState) -> Router<S> {
    let tracklists_sub_routes = Router::new()
        .route("/artists", get(routes::tracklists::artists::artists))
        .route(
            "/mix-series",
            get(routes::tracklists::mix_series::mix_series),
        )
        .route("/songs", get(routes::tracklists::songs::songs))
        .route("/docs", get(routes::tracklists::docs::docs));

    Router::new()
        .route("/tracklists", get(routes::tracklists::tracklists))
        .nest("/tracklists", tracklists_sub_routes)
        .with_state(state)
}
