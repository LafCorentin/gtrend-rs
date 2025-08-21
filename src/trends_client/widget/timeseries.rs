use serde::Deserialize;

use super::Text;

/// Google trend timeseries widget
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Timeseries {
    pub default: DefaultTimeSeries,
}

/// Subpart of Google trend [`Timeseries`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultTimeSeries {
    pub averages: Vec<u32>,
    pub timeline_data: Vec<TimeLineBit>,
}

/// Subpart of Google trend [`Timeseries`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TimeLineBit {
    pub axis_note: Option<Text>,
    pub formatted_axis_time: String,
    pub formatted_time: String,
    pub formatted_value: Vec<String>,
    pub has_data: Vec<bool>,
    pub time: String,
    pub value: Vec<u32>,
}
