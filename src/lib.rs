pub mod enums;
mod error;
mod trends_client;

pub use error::Error;
pub use trends_client::{
    ComparaisonElem, DEFAULT_ADDRESS, ExploreClient, ExploreResult, GeoMap, RelatedQueries,
    RelatedTopics, Request, Timeseries, TrendsClient, WidgetCategory, WidgetKeyword,
};
