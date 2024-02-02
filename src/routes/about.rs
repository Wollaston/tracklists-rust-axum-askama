use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/about.html")]
pub struct AboutTemplate;

pub async fn about() -> impl IntoResponse {
    AboutTemplate
}
