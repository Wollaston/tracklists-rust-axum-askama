use askama::Template;

pub mod artists;

#[derive(Template)]
#[template(path = "routes/tracklists.html")]
pub struct TracklistsTemplate {}

pub async fn tracklists() -> TracklistsTemplate {
    TracklistsTemplate {}
}
