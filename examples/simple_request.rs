use gtrend_rs::{
    ComparaisonElem, Request, TrendsClient, WidgetKeyword,
    enums::{Category, Country, Period, PredefinedPeriod, Property},
};

#[tokio::main]
async fn main() {
    let simple_request = Request::new(
        vec![ComparaisonElem {
            keyword: "cough".to_string(),
            geo: Country::ALL,
            time: Period::Predefined(PredefinedPeriod::OneYear),
        }],
        Category::RespiratoryConditions,
        Property::Web,
    )
    .unwrap();

    let client = TrendsClient::try_default().await.unwrap();
    let explore_client = client.explore(simple_request).await.unwrap();

    let stats = explore_client
        .get_timeseries(WidgetKeyword::All)
        .await
        .unwrap();

    println!("{:?}", stats);
}
