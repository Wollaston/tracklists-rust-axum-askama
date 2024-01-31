use askama::Template;

#[derive(Template)]
#[template(path = "routes/tracklists/mix-series.html")]
pub struct MixSeriesTemplate {}

pub async fn mix_series() -> MixSeriesTemplate {
    MixSeriesTemplate {}
}
