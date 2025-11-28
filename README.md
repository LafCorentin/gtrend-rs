# gtrend-rs

A Rust client for Google Trends

## Overview

gtrend-rs is a Rust crate providing a convenient way to access Google Trends data programmatically.
It aims to offer a simple and idiomatic API to fetch search trends, including interest over time, regional data, and related topics.

## Features

- Query Google Trends programmatically in Rust
- Support for interest over time, regional breakdowns, and related topics
- Example programs included in the examples/ directory

## Installation

Add the dependency :
```sh
cargo add gtrend-rs
```

## Usage

The google API works by first sending a `Request` containing keywords, period of time etc. Then it returns accesses to widgets containing the interesting data. This first return is represented in the crate as `ExploreClient`, from which one can access the widgets.

```rust
let simple_request = Request::new(
    vec![ComparaisonElem {
        keyword: "rust",
        geo: Country::ALL,
        time: Period::Predefined(PredefinedPeriod::OneYear),
    }],
    Category::Programming,
    Property::Web,
).unwrap();

let client = TrendsClient::try_default().await.unwrap();
let explore_client = client.explore(simple_request).await.unwrap();

let stats = explore_client
    .get_timeseries(&WidgetKeyword::All)
    .await
    .unwrap();
```

Each widget has its own `struct`, but can also be acceeded as `serde_json::Value` in case anyone need to access raw data.

> Note: The Google Trends API is not officially public. This crate does not yet provide authentication. Excessive queries may result in temporary rate limiting or blocking by Google.

## Contributing

Contributions are welcome. Feel free to open issues or submit pull requests to improve the library.

## License

This project is licensed under the [Apache 2.0 License](./LICENSE).