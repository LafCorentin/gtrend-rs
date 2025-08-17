//! Represent a Google Trend Property

use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum Property {
    #[serde(rename = "")]
    Web,
    Images,
    News,
    Froogle,
    Youtube,
}
