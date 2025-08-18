use std::collections::HashMap;

use serde::Deserialize;

use crate::{
    error::{Error, Result},
    trends_client::{TrendsClient, geo_map::GeoMap, sanitize_google_json, timeseries::Timeseries},
};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq, Hash)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WidgetCategory {
    Timeseries,
    GeoMap,
    RelatedTopics,
    RelatedQueries,
}

impl TryFrom<&str> for WidgetCategory {
    fn try_from(value: &str) -> Result<Self> {
        if value.contains("TIMESERIES") {
            Ok(WidgetCategory::Timeseries)
        } else if value.contains("GEO_MAP") {
            Ok(WidgetCategory::GeoMap)
        } else if value.contains("RELATED_TOPICS") {
            Ok(WidgetCategory::RelatedTopics)
        } else if value.contains("RELATED_QUERIES") {
            Ok(WidgetCategory::RelatedQueries)
        } else {
            Err(Error::UnexpectedResponse(format!(
                "Irregular widget category: {value}"
            )))
        }
    }

    type Error = Error;
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum WidgetKeyword {
    All,
    Keyword(String),
}

#[derive(Debug, Deserialize)]
pub struct Widget {
    token: String,
    request: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ExploreResult {
    widgets: Vec<serde_json::Value>,
    //skipping many items here
}

#[derive(Debug)]
pub struct ExploreClient {
    trends_client: TrendsClient,
    widgets: HashMap<(WidgetKeyword, WidgetCategory), Widget>,
}

impl ExploreClient {
    pub fn new(trends_client: TrendsClient, explore_result: ExploreResult) -> Result<Self> {
        let mut widgets = HashMap::new();
        let mut keyword = WidgetKeyword::All;

        for widget in &explore_result.widgets {
            match widget
                .get("template")
                .ok_or_else(|| Error::UnexpectedResponse(format!("Irregular widget: {widget}")))?
                .as_str()
                .ok_or_else(|| Error::UnexpectedResponse(format!("Irregular widget: {widget}")))?
            {
                "fe" => {
                    let id = widget
                        .get("id")
                        .ok_or_else(|| {
                            Error::UnexpectedResponse(format!("Irregular widget: {widget}"))
                        })?
                        .as_str()
                        .ok_or_else(|| {
                            Error::UnexpectedResponse(format!("Irregular widget: {widget}"))
                        })?;

                    if id.contains("_note") {
                        continue;
                    }

                    let category = WidgetCategory::try_from(id)?;

                    widgets.insert(
                        (keyword.clone(), category),
                        serde_json::from_value(widget.clone())?,
                    );
                }
                "fe_explore" => {
                    keyword = WidgetKeyword::Keyword(
                        widget
                            .get("text")
                            .ok_or_else(|| {
                                Error::UnexpectedResponse(format!("Irregular widget: {widget}"))
                            })?
                            .get("text")
                            .ok_or_else(|| {
                                Error::UnexpectedResponse(format!("Irregular widget: {widget}"))
                            })?
                            .as_str()
                            .ok_or_else(|| {
                                Error::UnexpectedResponse(format!("Irregular widget: {widget}"))
                            })?
                            .to_string(),
                    );
                }
                _ => {
                    return Err(Error::UnexpectedResponse(format!(
                        "Irregular widget: {widget}"
                    )));
                }
            }
        }

        Ok(Self {
            trends_client,
            widgets,
        })
    }

    pub fn available_widgets(&self) -> Vec<(WidgetKeyword, WidgetCategory)> {
        self.widgets.keys().cloned().collect()
    }

    async fn get_widget(&self, keyword: WidgetKeyword, category: WidgetCategory) -> Result<String> {
        let widget = self
            .widgets
            .get(&(keyword.clone(), category))
            .ok_or_else(|| {
                Error::Params(format!(
                    "No widget for category {category:?} and keyword {keyword:?}"
                ))
            })?;
        let end_url = match category {
            WidgetCategory::Timeseries => "api/widgetdata/multiline",
            WidgetCategory::GeoMap => "api/widgetdata/comparedgeo",
            WidgetCategory::RelatedTopics => "api/widgetdata/relatedsearches",
            WidgetCategory::RelatedQueries => "api/widgetdata/relatedsearches",
        };
        self.trends_client
            .get(
                end_url,
                &serde_json::to_string(&widget.request)?,
                Some(&widget.token),
            )
            .await
    }

    pub async fn get_widget_as_json(
        &self,
        keyword: WidgetKeyword,
        category: WidgetCategory,
    ) -> Result<serde_json::Value> {
        let widget = self.get_widget(keyword, category).await?;
        let cleaned_widget = sanitize_google_json(&widget);
        serde_json::from_str(cleaned_widget).map_err(Error::from)
    }

    pub async fn get_timeseries(&self, keyword: WidgetKeyword) -> Result<Timeseries> {
        let content = self.get_widget(keyword, WidgetCategory::Timeseries).await?;
        serde_json::from_str(sanitize_google_json(&content)).map_err(Error::from)
    }

    pub async fn get_geomap(&self, keyword: WidgetKeyword) -> Result<GeoMap> {
        let content = self.get_widget(keyword, WidgetCategory::GeoMap).await?;
        serde_json::from_str(sanitize_google_json(&content)).map_err(Error::from)
    }
}
