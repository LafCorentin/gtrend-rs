//! Represent a Google Trend Property

use serde::Serialize;

/// Represent a Google Trend Property
#[derive(Debug, Serialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "kebab-case")]
pub enum Property {
    #[serde(rename = "")]
    Web,
    Images,
    News,
    Froogle,
    Youtube,
}
