use askama::Template;
use axum::{routing::get, Router};
use tower_http::services::{ServeDir, ServeFile};

#[derive(Template)]
#[template(path = "home.html")]
struct HomeTemplate<'a> {
    example: &'a str,
}

async fn home() -> HomeTemplate<'static> {
    HomeTemplate {
        example: "Example content loaded from route!",
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(home))
        .nest_service("/assets", ServeDir::new("public/assets/"))
        .nest_service("/css", ServeDir::new("style/"))
        .route_service("/favicon.ico", ServeFile::new("public/favicon.ico"));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}
