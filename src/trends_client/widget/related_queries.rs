use derive_getters::Getters;
use serde::Deserialize;

/// Google trend related queries widget
#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedQueries {
    default: DefaultRelatedQueries,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultRelatedQueries {
    ranked_list: Vec<RankedKeyword>,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RankedKeyword {
    ranked_keyword: Vec<RelatedKeyword>,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Getters, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedKeyword {
    formatted_value: String,
    has_data: Option<bool>,
    link: String,
    query: String,
    value: u32,
}
