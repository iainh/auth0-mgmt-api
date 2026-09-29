use thiserror::Error;

use crate::types::JobId;

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

    /// A job did not finish before the caller's deadline.
    #[error("Timed out waiting for job {job_id} (last status: {last_status})")]
    JobTimeout { job_id: JobId, last_status: String },
}

pub type Result<T> = std::result::Result<T, Auth0Error>;

#[derive(Debug, serde::Deserialize)]
pub(crate) struct Auth0ApiError {
    pub message: Option<String>,
    pub error: Option<String>,
    #[serde(rename = "errorCode")]
    pub error_code: Option<String>,
}
