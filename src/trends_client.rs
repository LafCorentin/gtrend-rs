use quick_xml::{Reader, events::Event};
use reqwest::{header::HeaderMap, Client, ClientBuilder};
use serde::{Deserialize, Serialize};

use crate::{
    enums::{category::Category, country::Country, lang::Lang, period::Period, property::Property},
    error::{Error, Result},
};

const DEFAULT_ADDRESS: &str = "https://trends.google.com/trends/api/";

pub enum TrendsEndpoint {
    Default,
    Custom(String),
}

pub struct TrendsClient {
    endpoint: String,
    client: Client,
    lang: Lang,
    country: Country,
}

impl TrendsClient {
    pub async fn new(endpoint: TrendsEndpoint) -> Result<Self> {

        let headers = HeaderMap::new();

        Ok(Self {
            endpoint: match endpoint {
                TrendsEndpoint::Default => DEFAULT_ADDRESS.to_string(),
                TrendsEndpoint::Custom(endpoint) => endpoint,
            },
            client: ClientBuilder::new().default_headers(headers).build()?,
            lang: Lang::EN,
            country: Country::US,
        })
    }

    pub async fn load(&self, request: Request) -> Result<ExploreResult> {
        let res = self
            .client
            .get(format!("{}{}", self.endpoint, "explore"))
            .query(&[
                ("hl", self.lang.to_string().as_str()),
                ("geo", self.country.to_string().as_str()),
                ("tz", "-120"),
                ("req", serde_json::to_string(&request)?.as_str()),
                ("tz", "-120"),
            ])
            .send()
            .await?;

        let result = res.text().await?;

        print!("returned : {result}");

        let try_explore_result: Result<ExploreResult> =
            serde_json::from_str(result.as_str()).map_err(|e| e.into());

        match try_explore_result {
            Ok(explore_result) => Ok(explore_result),
            Err(_) => response_problem(&result),
        }
    }
}

fn response_problem(result: &str) -> Result<ExploreResult> {
    let mut buf = Vec::new();
    let mut reader = Reader::from_str(result);
    reader.config_mut().trim_text(true);

    loop {
        match reader.read_event_into(&mut buf) {
            Ok(Event::Start(e)) if e.name().as_ref() == b"title" => {
                if let Ok(Event::Text(text)) = reader.read_event_into(&mut buf) {
                    let title = text.decode()?.into_owned();
                    return Err(Error::API(title));
                }
            }
            Ok(Event::Eof) => return Err(Error::unexpected_response_error(result)),
            Ok(_) => (),
            Err(_) => break,
        }
        buf.clear();
    }

    Err(Error::UnexpectedResponse(format!("Seems not to be xml: {result}")))
}

#[derive(Debug, Deserialize)]
pub struct ExploreResult {
    token: String,
    request: String,
    lang: String,
}

#[derive(Debug, Serialize)]
pub struct Request {
    comparaison_items: Vec<ComparaisonItem>,
    category: Category,
    property: Property,
}

impl Request {
    pub fn new(
        comparaison_items: Vec<ComparaisonItem>,
        category: Category,
        property: Property,
    ) -> Result<Self> {
        if comparaison_items.is_empty() {
            return Err(Error::params_error("1 comparaison item minimum"));
        }

        if comparaison_items.len() > 5 {
            return Err(Error::params_error("5 comparaison items maximum"));
        }

        Ok(Self {
            comparaison_items,
            category,
            property,
        })
    }
}

#[derive(Debug, Serialize)]
pub struct ComparaisonItem {
    pub keyword: String,
    pub geo: Country,
    pub time: Period,
}

#[cfg(test)]
mod tests {
    use crate::enums::period::PredefinedPeriod;

    use super::*;

    #[tokio::test]
    async fn test_request() {
        let request = Request::new(
            vec![ComparaisonItem {
                keyword: "test".to_string(),
                geo: Country::US,
                time: Period::Predefined(PredefinedPeriod::OneYear),
            }],
            Category::All,
            Property::Web,
        )
        .unwrap();

        println!("request: {}", serde_json::to_string(&request).unwrap());

        let client = TrendsClient::new(TrendsEndpoint::Default);
        let res = client.load(request).await.unwrap();

        println!("{:#?}", res);
    }

    #[tokio::test]
    async fn reponse_problem() {
        let result = "<meta charset= utf-8>
<meta name= viewport content=\"initial-scale=1, minimum-scale=1, width=device-width\">
<title>Error 400 (Bad Request)!!1</title>";

        let err = response_problem(result).err().unwrap();

        assert_eq!(err, Error::api_error("Error 400 (Bad Request)!!1"));
    }
}
