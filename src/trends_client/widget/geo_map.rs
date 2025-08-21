use serde::Deserialize;

/// Google trend GeoMap Widget
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    pub default: DefaultGeoMap,
}

/// Subpart of Google trend [`GeoMap`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeoMap {
    pub geo_map_data: Vec<GeoMapItem>,
}

/// Subpart of Google trend [`GeoMap`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoMapItem {
    pub formatted_value: Vec<String>,
    pub geo_code: String,
    pub geo_name: String,
    pub has_data: Vec<bool>,
    pub max_value_index: u32,
    pub value: Vec<u32>,
}
