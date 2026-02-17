use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::sync::Arc;
use tokio::sync::{RwLock, Semaphore};
use url::Url;

use crate::error::{Auth0ApiError, Auth0Error, Result};

#[cfg(feature = "clients")]
use crate::api::clients::ClientsApi;
#[cfg(feature = "connections")]
use crate::api::connections::ConnectionsApi;
#[cfg(feature = "logs")]
use crate::api::logs::LogsApi;
#[cfg(feature = "users")]
use crate::api::users::UsersApi;

#[derive(Clone)]
pub struct ManagementClient {
    http: Client,
    base_url: Url,
    credentials: Credentials,
    token: Arc<RwLock<Option<TokenInfo>>>,
    token_refresh_semaphore: Arc<Semaphore>,
    retry_config: RetryConfig,
}

/// Configuration for token refresh retry behavior with exponential backoff.
#[derive(Clone, Debug)]
pub struct RetryConfig {
    /// Maximum number of retry attempts (not including the initial attempt).
    pub max_retries: u32,
    /// Initial delay before the first retry.
    pub initial_delay: std::time::Duration,
    /// Maximum delay between retries.
    pub max_delay: std::time::Duration,
    /// Multiplier applied to delay after each retry.
    pub multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: std::time::Duration::from_millis(100),
            max_delay: std::time::Duration::from_secs(10),
            multiplier: 2.0,
        }
    }
}

#[derive(Clone)]
struct Credentials {
    client_id: String,
    client_secret: SecretString,
    audience: String,
}

#[derive(Clone)]
struct TokenInfo {
    access_token: String,
    expires_at: std::time::Instant,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: u64,
    #[allow(dead_code)]
    token_type: String,
}

#[derive(Serialize)]
struct TokenRequest<'a> {
    grant_type: &'static str,
    client_id: &'a str,
    client_secret: &'a str,
    audience: &'a str,
}

impl ManagementClient {
    pub fn builder() -> ManagementClientBuilder {
        ManagementClientBuilder::default()
    }

    pub(crate) fn base_url(&self) -> &Url {
        &self.base_url
    }

    async fn get_token(&self) -> Result<String> {
        {
            let token = self.token.read().await;
            if let Some(info) = token.as_ref()
                && info.expires_at > std::time::Instant::now()
            {
                return Ok(info.access_token.clone());
            }
        }

        let _permit = self
            .token_refresh_semaphore
            .acquire()
            .await
            .map_err(|_| Auth0Error::Configuration("Token refresh semaphore closed".into()))?;

        {
            let token = self.token.read().await;
            if let Some(info) = token.as_ref()
                && info.expires_at > std::time::Instant::now()
            {
                return Ok(info.access_token.clone());
            }
        }

        let token_url = self.base_url.join("oauth/token")?;

        let mut last_error = None;
        let mut delay = self.retry_config.initial_delay;

        for attempt in 0..=self.retry_config.max_retries {
            if attempt > 0 {
                tokio::time::sleep(delay).await;
                delay = std::cmp::min(
                    self.retry_config.max_delay,
                    std::time::Duration::from_secs_f64(
                        delay.as_secs_f64() * self.retry_config.multiplier,
                    ),
                );
            }

            let request = TokenRequest {
                grant_type: "client_credentials",
                client_id: &self.credentials.client_id,
                client_secret: self.credentials.client_secret.expose_secret(),
                audience: &self.credentials.audience,
            };

            let result = self
                .http
                .post(token_url.clone())
                .json(&request)
                .send()
                .await;

            let response = match result {
                Ok(resp) => resp,
                Err(e)
                    if Self::is_retryable_error(&e) && attempt < self.retry_config.max_retries =>
                {
                    last_error = Some(Auth0Error::Http(e));
                    continue;
                }
                Err(e) => return Err(Auth0Error::Http(e)),
            };

            let status = response.status();

            if status.is_success() {
                let token_response: TokenResponse = response.json().await?;
                let expires_at = std::time::Instant::now()
                    + std::time::Duration::from_secs(token_response.expires_in.saturating_sub(60));

                let mut token = self.token.write().await;
                *token = Some(TokenInfo {
                    access_token: token_response.access_token.clone(),
                    expires_at,
                });

                return Ok(token_response.access_token);
            }

            if Self::is_retryable_status(status.as_u16()) && attempt < self.retry_config.max_retries
            {
                if status.as_u16() == 429
                    && let Some(retry_after) = response
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| v.parse::<u64>().ok())
                {
                    delay = std::time::Duration::from_secs(retry_after);
                }
                last_error = Some(Auth0Error::Authentication {
                    message: format!("Token request failed with status {}", status.as_u16()),
                });
                continue;
            }

            let error: Auth0ApiError = response.json().await?;
            return Err(Auth0Error::Authentication {
                message: error.message.unwrap_or(error.error.unwrap_or_default()),
            });
        }

        Err(last_error.unwrap_or_else(|| Auth0Error::Authentication {
            message: "Token refresh failed after retries".into(),
        }))
    }

    fn is_retryable_error(error: &reqwest::Error) -> bool {
        error.is_timeout() || error.is_connect() || error.is_request()
    }

    fn is_retryable_status(status: u16) -> bool {
        status == 429 || status == 502 || status == 503 || status == 504
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let token = self.get_token().await?;
        let response = self.http.get(url).bearer_auth(&token).send().await?;

        self.handle_response(response).await
    }

    pub(crate) async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        url: Url,
        body: &B,
    ) -> Result<T> {
        let token = self.get_token().await?;
        let response = self
            .http
            .post(url)
            .bearer_auth(&token)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub(crate) async fn patch<T: DeserializeOwned, B: Serialize>(
        &self,
        url: Url,
        body: &B,
    ) -> Result<T> {
        let token = self.get_token().await?;
        let response = self
            .http
            .patch(url)
            .bearer_auth(&token)
            .json(body)
            .send()
            .await?;

        self.handle_response(response).await
    }

    pub(crate) async fn delete(&self, url: Url) -> Result<()> {
        let token = self.get_token().await?;
        let response = self.http.delete(url).bearer_auth(&token).send().await?;

        if response.status().is_success() {
            Ok(())
        } else {
            self.handle_error(response).await
        }
    }

    async fn handle_response<T: DeserializeOwned>(&self, response: reqwest::Response) -> Result<T> {
        if response.status().is_success() {
            Ok(response.json().await?)
        } else {
            self.handle_error(response).await
        }
    }

    async fn handle_error<T>(&self, response: reqwest::Response) -> Result<T> {
        let status = response.status().as_u16();

        if status == 429 {
            let retry_after = response
                .headers()
                .get("retry-after")
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse().ok());
            return Err(Auth0Error::RateLimited { retry_after });
        }

        let error: Auth0ApiError = response.json().await.unwrap_or(Auth0ApiError {
            error: Some("Unknown error".into()),
            message: None,
            error_code: None,
        });

        Err(Auth0Error::Api {
            status,
            message: error.message.unwrap_or(error.error.unwrap_or_default()),
            error_code: error.error_code,
        })
    }

    #[cfg(feature = "users")]
    pub fn users(&self) -> UsersApi<'_> {
        UsersApi::new(self)
    }

    #[cfg(feature = "clients")]
    pub fn clients(&self) -> ClientsApi<'_> {
        ClientsApi::new(self)
    }

    #[cfg(feature = "connections")]
    pub fn connections(&self) -> ConnectionsApi<'_> {
        ConnectionsApi::new(self)
    }

    #[cfg(feature = "logs")]
    pub fn logs(&self) -> LogsApi<'_> {
        LogsApi::new(self)
    }
}

#[derive(Default, Clone)]
pub struct ManagementClientBuilder {
    domain: Option<String>,
    client_id: Option<String>,
    client_secret: Option<SecretString>,
    audience: Option<String>,
    retry_config: Option<RetryConfig>,
}

impl ManagementClientBuilder {
    pub fn domain(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    pub fn client_id(mut self, client_id: impl Into<String>) -> Self {
        self.client_id = Some(client_id.into());
        self
    }

    pub fn client_secret(mut self, client_secret: impl Into<String>) -> Self {
        self.client_secret = Some(SecretString::from(client_secret.into()));
        self
    }

    pub fn audience(mut self, audience: impl Into<String>) -> Self {
        self.audience = Some(audience.into());
        self
    }

    pub fn retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = Some(config);
        self
    }

    pub fn build(self) -> Result<ManagementClient> {
        let domain = self
            .domain
            .ok_or_else(|| Auth0Error::Configuration("domain is required".into()))?;

        let client_id = self
            .client_id
            .ok_or_else(|| Auth0Error::Configuration("client_id is required".into()))?;

        let client_secret = self
            .client_secret
            .ok_or_else(|| Auth0Error::Configuration("client_secret is required".into()))?;

        let base_url = if domain.starts_with("http://") || domain.starts_with("https://") {
            Url::parse(&domain)?
        } else {
            Url::parse(&format!("https://{}/", domain))?
        };

        let audience = self
            .audience
            .unwrap_or_else(|| format!("{}api/v2/", base_url));

        let http = Client::builder()
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;

        Ok(ManagementClient {
            http,
            base_url,
            credentials: Credentials {
                client_id,
                client_secret,
                audience,
            },
            token: Arc::new(RwLock::new(None)),
            token_refresh_semaphore: Arc::new(Semaphore::new(1)),
            retry_config: self.retry_config.unwrap_or_default(),
        })
    }
}
