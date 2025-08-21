use derive_getters::Getters;
use serde::Deserialize;

pub mod geo_map;
pub mod related_queries;
pub mod related_topics;
pub mod timeseries;

/// Text object in Google Trends structures
#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Text {
    text: String,
}
