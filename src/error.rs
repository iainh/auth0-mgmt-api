use thiserror::Error;

#[derive(Error, Debug)]
pub enum Auth0Error {
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    #[error("Failed to parse JSON: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Invalid URL: {0}")]
    Url(#[from] url::ParseError),

    #[error("Authentication failed: {message}")]
    Authentication { message: String },

    #[error("API error ({status}): {message}")]
    Api {
        status: u16,
        message: String,
        error_code: Option<String>,
    },

    #[error("Rate limited: retry after {retry_after:?} seconds")]
    RateLimited { retry_after: Option<u64> },

    #[error("Configuration error: {0}")]
    Configuration(String),
}

pub type Result<T> = std::result::Result<T, Auth0Error>;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Auth0ApiError {
    #[serde(alias = "error")]
    pub message: String,
    #[serde(alias = "error_description")]
    pub description: Option<String>,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
}
