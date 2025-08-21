use serde::Deserialize;

/// Google trend related queries widget
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedQueries {
    pub default: DefaultRelatedQueries,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultRelatedQueries {
    pub ranked_list: Vec<RankedKeyword>,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RankedKeyword {
    pub ranked_keyword: Vec<RelatedKeyword>,
}

/// Subpart of Google trend [`RelatedQueries`]
#[derive(Debug, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RelatedKeyword {
    pub formatted_value: String,
    pub has_data: Option<bool>,
    pub link: String,
    pub query: String,
    pub value: u32,
}
