use reqwest::Client;
use secrecy::{ExposeSecret, SecretString};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;
use url::Url;

use crate::error::{Auth0ApiError, Auth0Error, Result};

#[cfg(feature = "clients")]
use crate::api::clients::ClientsApi;
#[cfg(feature = "connections")]
use crate::api::connections::ConnectionsApi;
#[cfg(feature = "users")]
use crate::api::users::UsersApi;

#[derive(Clone)]
pub struct ManagementClient {
    http: Client,
    base_url: Url,
    credentials: Credentials,
    token: Arc<RwLock<Option<TokenInfo>>>,
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

        let mut token = self.token.write().await;
        if let Some(info) = token.as_ref()
            && info.expires_at > std::time::Instant::now()
        {
            return Ok(info.access_token.clone());
        }

        let token_url = self.base_url.join("oauth/token")?;
        let request = TokenRequest {
            grant_type: "client_credentials",
            client_id: &self.credentials.client_id,
            client_secret: self.credentials.client_secret.expose_secret(),
            audience: &self.credentials.audience,
        };

        let response = self.http.post(token_url).json(&request).send().await?;

        if !response.status().is_success() {
            let error: Auth0ApiError = response.json().await?;
            return Err(Auth0Error::Authentication {
                message: error.description.unwrap_or(error.message),
            });
        }

        let token_response: TokenResponse = response.json().await?;
        let expires_at =
            std::time::Instant::now() + std::time::Duration::from_secs(token_response.expires_in - 60);

        *token = Some(TokenInfo {
            access_token: token_response.access_token.clone(),
            expires_at,
        });

        Ok(token_response.access_token)
    }

    pub(crate) async fn get<T: DeserializeOwned>(&self, url: Url) -> Result<T> {
        let token = self.get_token().await?;
        let response = self
            .http
            .get(url)
            .bearer_auth(&token)
            .send()
            .await?;

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
        let response = self
            .http
            .delete(url)
            .bearer_auth(&token)
            .send()
            .await?;

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
            message: "Unknown error".to_string(),
            description: None,
            error_code: None,
        });

        Err(Auth0Error::Api {
            status,
            message: error.description.unwrap_or(error.message),
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
}

#[derive(Default)]
pub struct ManagementClientBuilder {
    domain: Option<String>,
    client_id: Option<String>,
    client_secret: Option<SecretString>,
    audience: Option<String>,
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

        let audience = self.audience.unwrap_or_else(|| {
            format!("{}api/v2/", base_url)
        });

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
        })
    }
}
