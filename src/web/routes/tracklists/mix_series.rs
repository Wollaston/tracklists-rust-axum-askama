use askama::Template;
use askama_axum::IntoResponse;

#[derive(Template)]
#[template(path = "routes/tracklists/mix-series.html")]
pub struct MixSeriesTemplate;

pub async fn mix_series_handler() -> impl IntoResponse {
    println!("->> {:<12} - mix_series_handler", "HANDLER");
    MixSeriesTemplate
}
