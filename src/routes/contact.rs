use askama::Template;

#[derive(Template)]
#[template(path = "routes/contact.html")]
pub struct ContactTemplate<'a> {
    pub contact: &'a str,
}

pub async fn contact() -> ContactTemplate<'static> {
    ContactTemplate {
        contact: "Example content loaded from contact!",
    }
}
