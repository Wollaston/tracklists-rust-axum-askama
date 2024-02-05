use askama_axum::IntoResponse;
use axum::http::Method;
use axum::http::Uri;
use axum::{extract::FromRef, middleware, response::Response, Json, Router};
use ctx::Ctx;
use model::ModelController;
use serde_json::json;
use tower_cookies::CookieManagerLayer;
use tower_http::{
    services::{ServeDir, ServeFile},
    trace::TraceLayer,
};

pub mod ctx;
pub mod db;
pub mod error;
pub mod log;
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
        .layer(middleware::from_fn_with_state(
            app_state.clone(),
            web::middleware::auth::mw_ctx_resolver,
        ))
        .layer(CookieManagerLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(app_state);

    let port = "127.0.0.1:8080";

    let listener = tokio::net::TcpListener::bind(port).await.unwrap();

    println!("->> Listening on {port}\n");
    axum::serve(listener, app).await.unwrap();
    Ok(())
}

async fn main_response_mapper(
    ctx: Option<Ctx>,
    uri: Uri,
    req_method: Method,
    res: Response,
) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    let uuid = uuid::Uuid::new_v4();

    // Get Server Error
    let service_error = res.extensions().get::<Error>();
    let client_status_error = service_error.map(|se| se.client_status_and_error());

    // If Client Error, build the response
    let error_response = client_status_error
        .as_ref()
        .map(|(status_code, client_error)| {
            let client_error_body = json!({
                "error" : {
                "type": client_error.as_ref(),
                "req_uuid": uuid.to_string(),
            }
            });

            println!("  ->> client_error_body: {client_error_body}");

            // Build the new response from the client_error_body
            (*status_code, Json(client_error_body)).into_response()
        });
    // Build and log the server log line.
    let client_error = client_status_error.unzip().1;
    // TODO: Need to hander if log_request fail (but should not fail request)
    let _ = log::log_request(uuid, req_method, uri, ctx, service_error, client_error).await;

    println!();
    error_response.unwrap_or(res)
}
