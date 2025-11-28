use chrono::NaiveDate;
use gtrend_rs::{
    ComparaisonElem, Request, TrendsClient, WidgetCategory,
    enums::{Category, Country, Date, DateHour, Period, PredefinedPeriod, Property},
};

#[tokio::test]
async fn global() {
    let keyword = "breath".to_string();
    let simple_request = Request::new(
        vec![ComparaisonElem {
            keyword: &keyword,
            geo: Country::US,
            time: Period::DatesHour(
                DateHour::from(
                    &NaiveDate::from_ymd_opt(2024, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
                DateHour::from(
                    &NaiveDate::from_ymd_opt(2024, 1, 1)
                        .unwrap()
                        .and_hms_opt(0, 0, 0)
                        .unwrap(),
                ),
            ),
        }],
        Category::RespiratoryConditions,
        Property::Web,
    )
    .unwrap();

    let keyword_1 = "google".to_string();
    let keyword_2 = "find".to_string();
    let request_diff_dates = Request::new(
        vec![
            ComparaisonElem {
                keyword: &keyword_1,
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: &keyword_2,
                geo: Country::US,
                time: Period::Dates(
                    Date::from(&NaiveDate::from_ymd_opt(2024, 1, 1).unwrap()),
                    Date::from(&NaiveDate::from_ymd_opt(2024, 1 + 8, 1).unwrap()),
                ),
            },
        ],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let request_diff_geo = Request::new(
        vec![
            ComparaisonElem {
                keyword: &keyword_1,
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: &keyword_2,
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
                keyword: &keyword,
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: &keyword_1,
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: &keyword_2,
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
        ],
        Category::All,
        Property::Web,
    )
    .unwrap();

    let client = TrendsClient::try_default().await.unwrap();

    test_request(simple_request, client.clone()).await;
    test_request(request_diff_dates, client.clone()).await;
    test_request(request_diff_geo, client.clone()).await;
    test_request(triple_request, client).await;
}

async fn test_request<'a>(request: Request<'a>, client: TrendsClient) {
    let explore_client: gtrend_rs::ExploreClient = client.explore(request).await.unwrap();

    for (keyword, category) in explore_client.available_widgets() {
        let data = explore_client
            .get_widget_as_json(&keyword, category)
            .await
            .unwrap();

        match category {
            WidgetCategory::Timeseries => {
                explore_client.get_timeseries(&keyword).await.unwrap();
            }
            WidgetCategory::GeoMap => {
                explore_client.get_geomap(&keyword).await.unwrap();
            }
            WidgetCategory::RelatedTopics => {
                assert_eq!(&data.to_string(), "{\"default\":{\"rankedList\":[]}}")
            }
            WidgetCategory::RelatedQueries => {
                explore_client.get_related_queries(&keyword).await.unwrap();
            }
        }
    }
}
