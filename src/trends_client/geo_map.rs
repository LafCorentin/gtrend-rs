use derive_getters::Getters;
use serde::Deserialize;


#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoMap {
    default: DefaultGeoMap
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultGeoMap {
    geo_map_data: Vec<GeoMapItem>
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct GeoMapItem {
    formatted_value: Vec<String>,
    geo_code: String,
    geo_name: String,
    has_data: Vec<bool>,
    max_value_index: u32,
    value: Vec<u32>
}

