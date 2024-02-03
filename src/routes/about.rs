use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/about.html")]
pub struct AboutTemplate;

pub async fn about_handler() -> impl IntoResponse {
    println!("->> {:<12} - about_handler", "HANDLER");
    AboutTemplate
}
