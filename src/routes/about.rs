use askama::Template;

#[derive(Template)]
#[template(path = "routes/about.html")]
pub struct AboutTemplate<'a> {
    pub about: &'a str,
}

pub async fn about() -> AboutTemplate<'static> {
    AboutTemplate {
        about: "Example content loaded from about!",
    }
}
