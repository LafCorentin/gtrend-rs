//! This module is responsible for fetching widgets from Google Trends.

use serde::Deserialize;
use std::collections::HashMap;

use crate::{
    error::{Error, Result},
    trends_client::{
        GeoMap, RelatedQueries, Timeseries, TrendsClient, response_problem, sanitize_google_json,
    },
};

/// Google trend Widget categories
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

/// Google trend Widget keywords
#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub enum WidgetKeyword {
    /// Main keyword if only one is given, otherwise all keywords
    All,

    /// Specific keyword when given multiple in the request
    Keyword(String),
}

#[derive(Debug, Deserialize)]
struct Widget {
    token: String,
    request: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub(crate) struct ExploreResult {
    widgets: Vec<serde_json::Value>,
    //skipping many items here
}

/// Client meant to fetch widgets from Google Trends
#[derive(Debug)]
pub struct ExploreClient {
    trends_client: TrendsClient,
    widgets: HashMap<(WidgetKeyword, WidgetCategory), Widget>,
}

impl ExploreClient {
    pub(crate) fn new(trends_client: TrendsClient, explore_result: ExploreResult) -> Result<Self> {
        let mut widgets = HashMap::new();
        let mut keyword = WidgetKeyword::All;

        for widget_json in &explore_result.widgets {
            match widget_json
                .get("template")
                .ok_or_irregular(widget_json)?
                .as_str()
                .ok_or_irregular(widget_json)?
            {
                "fe" => {
                    let id = widget_json
                        .get("id")
                        .ok_or_irregular(widget_json)?
                        .as_str()
                        .ok_or_irregular(widget_json)?;

                    if id.contains("_note") {
                        continue;
                    }

                    let category = WidgetCategory::try_from(id)?;
                    let widget: Widget = serde_json::from_value(widget_json.clone())?;
                    widgets.insert((keyword.clone(), category), widget);
                }
                "fe_explore" => {
                    keyword = WidgetKeyword::Keyword(
                        widget_json
                            .get("text")
                            .ok_or_irregular(widget_json)?
                            .get("text")
                            .ok_or_irregular(widget_json)?
                            .as_str()
                            .ok_or_irregular(widget_json)?
                            .to_string(),
                    );
                }
                _ => {
                    return Err(Error::UnexpectedResponse(format!(
                        "Irregular widget: {widget_json}"
                    )));
                }
            }
        }

        Ok(Self {
            trends_client,
            widgets,
        })
    }

    /// Returns all available widgets
    pub fn available_widgets(&self) -> Vec<(WidgetKeyword, WidgetCategory)> {
        self.widgets.keys().cloned().collect()
    }

    fn get_widget(&self, keyword: WidgetKeyword, category: WidgetCategory) -> Result<&Widget> {
        self.widgets
            .get(&(keyword.clone(), category))
            .ok_or_else(|| {
                Error::Params(format!(
                    "No widget for category {category:?} and keyword {keyword:?}"
                ))
            })
    }

    /// Returns the request made by Google service for the given widget
    pub fn get_widget_request(
        &self,
        keyword: WidgetKeyword,
        category: WidgetCategory,
    ) -> Result<&serde_json::Value> {
        let widget = self.get_widget(keyword, category)?;
        Ok(&widget.request)
    }

    async fn fetch_widget<T: for<'a> Deserialize<'a>>(
        &self,
        keyword: WidgetKeyword,
        category: WidgetCategory,
    ) -> Result<T> {
        let widget = self.get_widget(keyword, category)?;
        let end_url = match category {
            WidgetCategory::Timeseries => "trends/api/widgetdata/multiline",
            WidgetCategory::GeoMap => "trends/api/widgetdata/comparedgeo",
            WidgetCategory::RelatedTopics => "trends/api/widgetdata/relatedsearches",
            WidgetCategory::RelatedQueries => "trends/api/widgetdata/relatedsearches",
        };

        let request = serde_json::to_string(&widget.request)?;

        let content = self
            .trends_client
            .get(end_url, &request, Some(&widget.token))
            .await?;

        serde_json::from_str(sanitize_google_json(&content))
            .map_err(|_| response_problem(&content, &request))
    }

    /// Returns the widget as a JSON object
    pub async fn get_widget_as_json(
        &self,
        keyword: WidgetKeyword,
        category: WidgetCategory,
    ) -> Result<serde_json::Value> {
        self.fetch_widget(keyword, category).await
    }

    /// Returns the timeseries as a [`Timeseries`] object
    pub async fn get_timeseries(&self, keyword: WidgetKeyword) -> Result<Timeseries> {
        self.fetch_widget(keyword, WidgetCategory::Timeseries).await
    }

    /// Returns the geomap as a [`GeoMap`] object
    pub async fn get_geomap(&self, keyword: WidgetKeyword) -> Result<GeoMap> {
        self.fetch_widget(keyword, WidgetCategory::GeoMap).await
    }

    /// Returns the related queries as a [`RelatedQueries`] object
    pub async fn get_related_queries(&self, keyword: WidgetKeyword) -> Result<RelatedQueries> {
        self.fetch_widget(keyword, WidgetCategory::RelatedQueries)
            .await
    }
}

trait OkOrIrregular<T> {
    fn ok_or_irregular(self, widget: &serde_json::Value) -> Result<T>;
}

impl<T> OkOrIrregular<T> for Option<T> {
    fn ok_or_irregular(self, widget: &serde_json::Value) -> Result<T> {
        self.ok_or_else(|| Error::UnexpectedResponse(format!("Irregular widget: {widget}")))
    }
}
