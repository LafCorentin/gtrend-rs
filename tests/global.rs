use gtrend_rs::{
    enums::{
        category::Category,
        country::Country,
        period::{Period, PredefinedPeriod},
        property::Property,
    },
    trends_client::{ComparaisonElem, Request, TrendsClient, TrendsEndpoint, WidgetKeyword},
};
use tokio::{fs::File, io::AsyncWriteExt as _};

#[tokio::test]
async fn global() {
    let simple_request = Request::new(
        vec![ComparaisonElem {
            keyword: "google".to_string(),
            geo: Country::US,
            time: Period::Predefined(PredefinedPeriod::OneYear),
        }],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let request_diff_dates = Request::new(
        vec![
            ComparaisonElem {
                keyword: "google".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: "find".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneDay),
            },
        ],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let request_diff_geo = Request::new(
        vec![
            ComparaisonElem {
                keyword: "google".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: "find".to_string(),
                geo: Country::FR,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
        ],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let triple_request = Request::new(
        vec![
            ComparaisonElem {
                keyword: "test".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: "google".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: "find".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
        ],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let client = TrendsClient::new(TrendsEndpoint::Default).await.unwrap();

    test_request(simple_request, client.clone()).await;
    test_request(request_diff_dates, client.clone()).await;
    test_request(request_diff_geo, client.clone()).await;
    test_request(triple_request, client).await;
}

async fn test_request(request: Request, client: TrendsClient) {
    let res = client.explore(request).await.unwrap();

    for (keyword, category) in res.available_widgets() {

        let _ = res
            .get_widget_as_json(keyword.clone(), category)
            .await
            .unwrap();

        match category {
            gtrend_rs::trends_client::WidgetCategory::Timeseries => {
                res.get_timeseries(keyword).await.unwrap();
            }
            gtrend_rs::trends_client::WidgetCategory::GeoMap => {
                res.get_geomap(keyword).await.unwrap();
            }
            gtrend_rs::trends_client::WidgetCategory::RelatedTopics => {}
            gtrend_rs::trends_client::WidgetCategory::RelatedQueries => {
                res.get_related_queries(keyword).await.unwrap();
            }
        }
    }
}
