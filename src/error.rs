
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Params error: {0}")]
    Params(String),

    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("API error: {0}")]
    API(String),

    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    #[error("Encoding error: {0}")]
    Encoding(#[from] quick_xml::encoding::EncodingError),
}

impl Error {
    pub fn params_error(message: &str) -> Self {
        Error::Params(message.to_string())
    }

    pub fn api_error(message: &str) -> Self {
        Error::API(message.to_string())
    }

    pub fn unexpected_response_error(message: &str) -> Self {
        Error::UnexpectedResponse(message.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}