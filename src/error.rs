/// [`Error`] is the main error type used in the crate
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error concerning params sent to the function
    #[error("Params error: {0}")]
    Params(String),

    /// Error while parsing json
    #[error("Json error: {0}")]
    Json(#[from] serde_json::Error),

    /// Error from reqwest crate
    #[error("Reqwest error: {0}")]
    Reqwest(#[from] reqwest::Error),

    /// Error coming from Google API
    #[error("API error: {0}")]
    API(String),

    /// Unexpected response from Google API
    #[error("Unexpected response: {0}")]
    UnexpectedResponse(String),

    /// Error while parsing xml
    #[error("Encoding error: {0}")]
    Encoding(#[from] quick_xml::encoding::EncodingError),
}

impl Error {
    pub(crate) fn params_error(message: &str) -> Self {
        Error::Params(message.to_string())
    }

    pub(crate) fn api_error(message: &str) -> Self {
        Error::API(message.to_string())
    }

    pub(crate) fn unexpected_response_error(message: &str) -> Self {
        Error::UnexpectedResponse(message.to_string())
    }
}

/// Local simplified [`Result`]
pub type Result<T> = std::result::Result<T, Error>;

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        self.to_string() == other.to_string()
    }
}
