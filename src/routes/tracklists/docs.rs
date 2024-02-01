use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/tracklists/docs.html")]
pub struct DocsTemplate;

pub async fn docs() -> impl IntoResponse {
    DocsTemplate
}
