use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Timeseries {
    default: DefaultTimeSeries,
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultTimeSeries {
    averages: Vec<u32>,
    timeline_data: Vec<TimeLineBit>,
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeLineBit {
    formatted_axis_time: String,
    formatted_time: String,
    formatted_value: Vec<String>,
    has_data: Vec<bool>,
    time: String,
    value: Vec<u32>,
}