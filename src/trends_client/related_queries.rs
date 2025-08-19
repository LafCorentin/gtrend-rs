use derive_getters::Getters;
use serde::Deserialize;

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedQueries {
    default: DefaultRelatedQueries,
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultRelatedQueries {
    ranked_list: Vec<RankedKeyword>,
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RankedKeyword {
    ranked_keyword: Vec<RelatedKeyword>,
}

#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedKeyword {
    formatted_value: String,
    has_data: Option<bool>,
    link: String,
    query: String,
    value: u32,
}
