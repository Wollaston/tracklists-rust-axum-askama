use askama::Template;

#[derive(Template)]
#[template(path = "routes/tracklists/artists.html")]
pub struct ArtistsTemplate {}

pub async fn artists() -> ArtistsTemplate {
    ArtistsTemplate {}
}
