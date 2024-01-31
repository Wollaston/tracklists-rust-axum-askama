use askama::Template;

pub mod artists;
pub mod docs;
pub mod mix_series;
pub mod songs;

#[derive(Template)]
#[template(path = "routes/tracklists/overview.html")]
pub struct TracklistsMainTemplate {}

pub async fn tracklists() -> TracklistsMainTemplate {
    TracklistsMainTemplate {}
}
