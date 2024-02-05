use askama::Template;
use askama_axum::IntoResponse;
use tracing::info;

#[derive(Template)]
#[template(path = "not_found.html")]
struct NotFoundTemplate;

pub async fn not_found_handler() -> impl IntoResponse {
    info!("{:<12} - not_found_handler", "HANDLER");
    NotFoundTemplate
}
