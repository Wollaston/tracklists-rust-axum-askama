use askama::Template;

#[derive(Template)]
#[template(path = "routes/tracklists.html")]
pub struct TracklistsTemplate<'a> {
    pub tracklists: &'a str,
}

pub async fn tracklists() -> TracklistsTemplate<'static> {
    TracklistsTemplate {
        tracklists: "Example content loaded from tracklists!",
    }
}
