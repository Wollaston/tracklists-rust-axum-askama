use askama::Template;

#[derive(Template)]
#[template(path = "routes/tracklists/docs.html")]
pub struct DocsTemplate {}

pub async fn docs() -> DocsTemplate {
    DocsTemplate {}
}
