mod explore_client;
mod widget;

use quick_xml::{Reader, events::Event};
use reqwest::{
    Client,
    cookie::Jar,
    header::{ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT},
};
use serde::Serialize;
use std::{fmt, sync::Arc};

use crate::error::{Error, Result};
use crate::{
    enums::{Category, Country, Lang, Period, Property},
    trends_client::explore_client::ExploreResult,
};

pub use crate::trends_client::{
    explore_client::{ExploreClient, WidgetCategory, WidgetKeyword},
    widget::{
        geo_map::GeoMap, related_queries::RelatedQueries, related_topics::RelatedTopics,
        timeseries::Timeseries,
    },
};

/// Default Google Trends address
pub const DEFAULT_ADDRESS: &str = "https://trends.google.com";

/// Google Trends client
#[derive(Debug, Clone)]
pub struct TrendsClient {
    endpoint: String,
    client: Client,
    lang: Lang,
    country: Country,
}

impl TrendsClient {
    pub async fn new(endpoint: String, lang: Lang, country: Country) -> Result<Self> {
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
            .get(format!("{}/trends", DEFAULT_ADDRESS))
            .send()
            .await?;

        Ok(Self {
            endpoint,
            client,
            lang,
            country,
        })
    }

    pub async fn try_default() -> Result<Self> {
        Self::new(DEFAULT_ADDRESS.to_string(), Lang::EN, Country::ALL).await
    }

    async fn get(&self, end_url: &str, req: &str, token: Option<&str>) -> Result<String> {
        self.client
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
            .await?
            .text()
            .await
            .map_err(Error::from)
    }

    /// Returns an [`ExploreClient`] from a [`Request`]
    ///
    /// The API is unstable when handling time ranges that combine very different scales.
    /// While long ranges (e.g. several months) are usually accepted, adding a small
    /// offset (e.g. a few hours) to the same range can cause the request to fail.
    pub async fn explore(&self, request: Request) -> Result<ExploreClient> {
        let json_body_unsanitize = self
            .get(
                "trends/api/explore",
                serde_json::to_string(&request)?.as_str(),
                None,
            )
            .await?;
        let json_body = sanitize_google_json(&json_body_unsanitize);

        let explore_result: ExploreResult =
            serde_json::from_str(json_body).map_err(|_| response_problem(json_body, &request))?;

        ExploreClient::new(self.clone(), explore_result)
    }
}

fn response_problem<T: fmt::Debug>(result: &str, request: &T) -> Error {
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

    if result.contains("Our systems have detected unusual traffic from your computer") {
        return Error::api_error("API rate limit exceeded");
    }

    if result.contains("The server cannot process the request because it is malformed.") {
        return Error::API(format!(
            "Malformed request, asked to not retry. Request: {:?}",
            request
        ));
    }

    Error::UnexpectedResponse(format!(
        "Unexpected response. Please send this log to https://github.com/LafCorentin/gtrend-rs/issues.\n Request : {request:#?}\n Complete error : {result}"
    ))
}

/// Google API returns json preceded by obstructing symbols.
/// This function removes them.
fn sanitize_google_json(raw: &str) -> &str {
    match raw.find(['{', '[']) {
        Some(pos) => &raw[pos..],
        None => raw, // au cas où c'est vraiment le chaos
    }
}

/// Google Trend request
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

/// Google Trend comparaison item
#[derive(Debug, Serialize)]
pub struct ComparaisonElem {
    pub keyword: String,
    pub geo: Country,
    pub time: Period,
}

impl ComparaisonElem {
    pub fn new(keyword: String, geo: Country, time: Period) -> Self {
        Self { keyword, geo, time }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const TEST_REQUEST: &str = "Test request";

    #[tokio::test]
    async fn reponse_xml_problem() {
        let result = "<meta charset= utf-8>
<meta name= viewport content=\"initial-scale=1, minimum-scale=1, width=device-width\">
<title>Error 400 (Bad Request)!!1</title>";

        let err = response_problem(result, &TEST_REQUEST);

        assert_eq!(err, Error::api_error("Error 400 (Bad Request)!!1"));
    }

    #[tokio::test]
    async fn response_too_many_requests_problem() {
        let result = "
            <hr noshade size=\"1\" style=\"color:#ccc; background-color:#ccc;\"><br>\n<form id=\"captcha-form\" action=\"index\"
                method=\"post\">\n<noscript>\n<div style=\"font-size:13px;\">\n In order to continue, please enable javascript
                        on your web browser.\n</div>\n</noscript>\n
                <script src=\"https://www.google.com/recaptcha/enterprise.js\" async defer></script>\n
                <script>var submitCallback = function (response) { document.getElementById('captcha-form').submit(); };</script>\n
                <div id=\"recaptcha\" class=\"g-recaptcha\" data-sitekey=\"6LfwuyUTAAAAAOAmoS0fdqijC2PbbdH4kjq62Y1b\"
                    data-callback=\"submitCallback\"
                    data-s=\"SHORTED\">
                </div>\n\n<input type='hidden' name='q'
                    value='SHORTED'><input
                    type=\"hidden\" name=\"continue\"
                    value=\"https://trends.google.com/trends/SHORTED\">\n
            </form>\n
            <hr noshade size=\"1\" style=\"color:#ccc; background-color:#ccc;\">\n\n<div style=\"font-size:13px; line-break:
                anywhere;\">\n<b>About this page</b><br><br>\n\nOur systems have detected unusual traffic from your computer
                network. This page checks to see if it&#39;s really you sending the requests, and not a robot. <a href=\"#\"
                    onclick=\"document.getElementById('infoDiv').style.display='block' ;\">Why did this happen?</a><br><br>\n\n
                <div id=\"infoDiv\" style=\"display:none; background-color:#eee; padding:10px; margin:0 0 15px 0;
                    line-height:1.4em;\">\nThis page appears when Google automatically detects requests coming from your
                    computer network which appear to be in violation of the <a href=\"//www.google.com/policies/terms/\">Terms
                        of Service</a>. The block will expire shortly after those requests stop. In the meantime, solving the
                    above CAPTCHA will let you continue to use our services.<br><br>This traffic may have been sent by malicious
                    software, a browser plug-in, or a script that sends automated requests. If you share your network
                    connection, ask your administrator for help &mdash; a different computer using the same IP address may be
                    responsible. <a href=\"//support.google.com/websearch/answer/86640\">Learn more</a><br><br>Sometimes you may
                    be asked to solve the CAPTCHA if you are using advanced terms that robots are known to use, or sending
                    requests very quickly.\n</div>\n\nIP address: 2a02:8434:ff02:3201:f820:6062:ea14:bbdd</div>\n
            </div>\n</body>\n

            </html>";
        let err = response_problem(result, &TEST_REQUEST);

        assert_eq!(err, Error::api_error("API rate limit exceeded"));
    }

    #[tokio::test]
    async fn response_malformed_requests_problem() {
        let result = "<main id=\"af-error-container\" role=\"main\"><a href=//www.google.com><span id=logo aria-label=Google role=img></span></a><p><b>400.</b> 
        <ins>That’s an error.</ins>
        <p>The server cannot process the request because it is malformed. It should not be retried. 
        <ins>That’s all we know.</ins></main>";
        let err = response_problem(result, &TEST_REQUEST);

        assert!(err.to_string().contains("Malformed request"),);
    }

    #[test]
    fn sanitize_google_json_test() {
        assert_eq!(
            sanitize_google_json(")}]\n{'a': 1, 'b': 2}"),
            "{'a': 1, 'b': 2}"
        );
        assert_eq!(sanitize_google_json(")}]\n[ 1, 2]"), "[ 1, 2]");
    }
}
