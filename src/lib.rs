//! # gtrend-rs
//! gtrend-rs is a Rust crate providing a convenient way to access Google Trends data programmatically.
//!
//! It aims to offer a simple and idiomatic API to fetch search trends, including interest over time, regional data, and related topics.

/// [`enums`] module contains all the enums used in the crate
pub mod enums;

/// [`error`] module contains the error types used in the crate
mod error;

/// [`trends_client`] module contains the main client struct
mod trends_client;

pub use error::Error;
pub use trends_client::{
    ComparaisonElem, DEFAULT_ADDRESS, ExploreClient, GeoMap, RelatedQueries, RelatedTopics,
    Request, Timeseries, TrendsClient, WidgetCategory, WidgetKeyword,
};
