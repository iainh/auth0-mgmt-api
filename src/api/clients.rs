use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::clients::{Client, CreateClientRequest, ListClientsParams, UpdateClientRequest};

pub struct ClientsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> ClientsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: Option<ListClientsParams>) -> Result<Vec<Client>> {
        let mut url = self.client.base_url().join("api/v2/clients")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    pub async fn get(&self, id: &str) -> Result<Client> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }

    pub async fn create(&self, request: CreateClientRequest) -> Result<Client> {
        let url = self.client.base_url().join("api/v2/clients")?;
        self.client.post(url, &request).await
    }

    pub async fn update(&self, id: &str, request: UpdateClientRequest) -> Result<Client> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id)))?;

        self.client.patch(url, &request).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id)))?;

        self.client.delete(url).await
    }

    pub async fn rotate_secret(&self, id: &str) -> Result<Client> {
        let url = self.client.base_url().join(&format!(
            "api/v2/clients/{}/rotate-secret",
            urlencoding::encode(id)
        ))?;

        self.client.post(url, &()).await
    }
}
