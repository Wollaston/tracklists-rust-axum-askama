use askama::Template;

#[derive(Template)]
#[template(path = "routes/tracklists/songs.html")]
pub struct SongsTemplate {}

pub async fn songs() -> SongsTemplate {
    SongsTemplate {}
}
