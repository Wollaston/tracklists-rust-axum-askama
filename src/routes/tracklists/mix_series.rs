use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/tracklists/mix-series.html")]
pub struct MixSeriesTemplate;

pub async fn mix_series() -> impl IntoResponse {
    MixSeriesTemplate
}
