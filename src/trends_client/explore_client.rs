use serde::Deserialize;

use crate::{error::{Error, Result}, trends_client::TrendsClient};

#[derive(Debug, Deserialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum WidgetId {
    Timeseries,
    GeoMap,
    RelatedTopics,
    RelatedQueries,
}

#[derive(Debug, Deserialize)]
pub struct Widget {
    id: WidgetId,
    token: String,
    request: serde_json::Value,
}

#[derive(Debug, Deserialize)]
pub struct ExploreResult {
    widgets: Vec<Widget>,
    //skipping many items here
}

impl ExploreResult {
    pub fn available_widgets(&self) -> Vec<WidgetId> {
        self.widgets.iter().map(|w| w.id).collect()
    }

    fn get_widget(&self, widget_id: WidgetId) -> Result<&Widget> {
        self.widgets
            .iter()
            .find(|w| w.id == widget_id)
            .ok_or_else(|| Error::Params(format!("Widget {:?} not found", widget_id)))
    }
}

#[derive(Debug)]
pub struct ExploreClient {
    trends_client: TrendsClient,
    explore_result: ExploreResult,
}

impl ExploreClient {
    pub fn new(trends_client: TrendsClient, explore_result: ExploreResult) -> Self {
        Self {
            trends_client,
            explore_result,
        }
    }

    async fn get_widget(&self, widget_id: WidgetId) -> Result<String> {
        let widget = self.explore_result.get_widget(widget_id)?;
        let end_url = match widget.id {
            WidgetId::Timeseries => "api/widgetdata/multiline",
            WidgetId::GeoMap => "api/widgetdata/comparedgeo",
            WidgetId::RelatedTopics => "api/widgetdata/relatedsearches",
            WidgetId::RelatedQueries => "api/widgetdata/relatedsearches",
        };
        self.trends_client
            .get(
                end_url,
                &serde_json::to_string(&widget.request)?,
                Some(&widget.token),
        ).await
    }

    pub async fn get_timeseries_as_json(&self) -> Result<serde_json::Value> {
        let timeseries = self.get_widget(WidgetId::Timeseries).await?;
        serde_json::from_str(timeseries.as_str()).map_err(Error::from)
    }

    pub async fn get_geo_map_as_json(&self) -> Result<serde_json::Value> {
        let geo_map = self.get_widget(WidgetId::GeoMap).await?;
        serde_json::from_str(geo_map.as_str()).map_err(Error::from)
    }

    pub async fn get_related_topics_as_json(&self) -> Result<serde_json::Value> {
        let related_topics = self.get_widget(WidgetId::RelatedTopics).await?;
        serde_json::from_str(related_topics.as_str()).map_err(Error::from)
    }

    pub async fn get_related_queries_as_json(&self) -> Result<serde_json::Value> {
        let related_queries = self.get_widget(WidgetId::RelatedQueries).await?;
        serde_json::from_str(related_queries.as_str()).map_err(Error::from)
    }
}

