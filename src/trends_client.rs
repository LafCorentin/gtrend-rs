mod explore_client;

use std::sync::Arc;

use quick_xml::{Reader, events::Event};
use reqwest::{
    Client, IntoUrl,
    cookie::Jar,
    header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::Serialize;

use crate::{
    enums::{category::Category, country::Country, lang::Lang, period::Period, property::Property},
    error::{Error, Result},
    trends_client::explore_client::{ExploreClient, ExploreResult},
};

const DEFAULT_ADDRESS: &str = "https://trends.google.com/trends";

pub enum TrendsEndpoint {
    Default,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct TrendsClient {
    endpoint: String,
    client: Client,
    lang: Lang,
    country: Country,
}

impl TrendsClient {
    pub async fn new(endpoint: TrendsEndpoint) -> Result<Self> {
        // Camouflage headers
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_static(
                "Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                               AppleWebKit/537.36 (KHTML, like Gecko) \
                               Chrome/128.0.0.0 Safari/537.36",
            ),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

        let jar = Arc::new(Jar::default());
        let client = reqwest::Client::builder()
            .cookie_provider(jar.clone())
            .build()?;

        // Setting up cookies
        client
            .get("https://trends.google.com/trends/")
            .send()
            .await?;

        Ok(Self {
            endpoint: match endpoint {
                TrendsEndpoint::Default => DEFAULT_ADDRESS.to_string(),
                TrendsEndpoint::Custom(endpoint) => endpoint,
            },
            client,
            lang: Lang::EN,
            country: Country::US,
        })
    }

    async fn get(&self, end_url: &str, req: &str, token: Option<&str>) -> Result<String> {
        let response = self
            .client
            .get(format!("{}/{}", self.endpoint, end_url))
            .query(&[
                ("hl", self.lang.to_string().as_str()),
                ("geo", self.country.to_string().as_str()),
                ("tz", "-120"),
                ("req", req),
                ("token", token.unwrap_or("")),
                ("tz", "-120"),
            ])
            .send()
            .await?;

        response.text().await.map_err(Error::from)
    }

    pub async fn explore(&self, request: Request) -> Result<ExploreClient> {
        let json_body_unsanitize = self
            .get(
                "api/explore",
                serde_json::to_string(&request)?.as_str(),
                None,
            )
            .await?;
        let json_body = sanitize_google_json(&json_body_unsanitize);

        print!("returned : {json_body}");

        let try_explore_result: Result<ExploreResult> =
            serde_json::from_str(json_body).map_err(|e| e.into());

        match try_explore_result {
            Ok(explore_result) => Ok(ExploreClient::new(self.clone(), explore_result)),
            Err(_) => Err(response_problem(json_body)),
        }
    }
}

fn response_problem(result: &str) -> Error {
    let mut buf = Vec::new();
    let mut reader = Reader::from_str(result);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"title" => {
                if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                    match text.decode() {
                        Ok(decoded) => {
                            let title = decoded.into_owned();
                            return Error::API(title);
                        }
                        Err(e) => return e.into(),
                    }
                }
            }
            Ok(Event::Eof) => return Error::unexpected_response_error(result),
            Ok(_) => (),
            Err(_) => break,
        }
        buf.clear();
    }

    Error::UnexpectedResponse(format!("Seems not to be xml: {result}"))
}

fn sanitize_google_json(raw: &str) -> &str {
    match raw.find(['{', '[']) {
        Some(pos) => &raw[pos..],
        None => raw, // au cas où c'est vraiment le chaos
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Request {
    comparison_item: Vec<ComparaisonElem>,
    category: Category,
    property: Property,
}

impl Request {
    pub fn new(
        comparison_item: Vec<ComparaisonElem>,
        category: Category,
        property: Property,
    ) -> Result<Self> {
        if comparison_item.is_empty() {
            return Err(Error::params_error("1 comparaison item minimum"));
        }

        if comparison_item.len() > 5 {
            return Err(Error::params_error("5 comparaison items maximum"));
        }

        Ok(Self {
            comparison_item,
            category,
            property,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ComparaisonElem {
    pub keyword: String,
    pub geo: Country,
    pub time: Period,
}

#[cfg(test)]
mod tests {
    use tokio::{fs::File, io::AsyncWriteExt};

    use crate::{enums::period::PredefinedPeriod, trends_client::explore_client::WidgetId};

    use super::*;

    #[tokio::test]
    async fn test_request() {
        let request = Request::new(
            vec![ComparaisonElem {
                keyword: "test".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            },
            ComparaisonElem {
                keyword: "google".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            }],
            Category::All,
            Property::Web,
        )
        .unwrap();

        let client = TrendsClient::new(TrendsEndpoint::Default).await.unwrap();
        let res = client.explore(request).await.unwrap();

        println!("{:#?}", res);

        let timeseries = res.get_timeseries_as_json().await.unwrap(); 
        let mut file = File::create("test/timeseries.json").await.unwrap();
        file.write_all(timeseries.to_string().as_bytes()).await.unwrap();

        let geomap = res.get_geo_map_as_json().await.unwrap();
        let mut file = File::create("test/geomap.json").await.unwrap();
        file.write_all(geomap.to_string().as_bytes()).await.unwrap();

        let related_topics = res.get_related_topics_as_json().await.unwrap();
        let mut file = File::create("test/related_topics.json").await.unwrap();
        file.write_all(related_topics.to_string().as_bytes()).await.unwrap();

        let related_queries = res.get_related_queries_as_json().await.unwrap();
        let mut file = File::create("test/related_queries.json").await.unwrap();
        file.write_all(related_queries.to_string().as_bytes()).await.unwrap();

    }

    #[tokio::test]
    async fn reponse_problem() {
        let result = "<meta charset= utf-8>
<meta name= viewport content=\"initial-scale=1, minimum-scale=1, width=device-width\">
<title>Error 400 (Bad Request)!!1</title>";

        let err = response_problem(result);

        assert_eq!(err, Error::api_error("Error 400 (Bad Request)!!1"));
    }
}
