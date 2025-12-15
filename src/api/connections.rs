use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::connections::{
    Connection, CreateConnectionRequest, ListConnectionsParams, UpdateConnectionRequest,
};

pub struct ConnectionsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> ConnectionsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: Option<ListConnectionsParams>) -> Result<Vec<Connection>> {
        let mut url = self.client.base_url().join("api/v2/connections")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    pub async fn get(&self, id: &str) -> Result<Connection> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }

    pub async fn create(&self, request: CreateConnectionRequest) -> Result<Connection> {
        let url = self.client.base_url().join("api/v2/connections")?;
        self.client.post(url, &request).await
    }

    pub async fn update(&self, id: &str, request: UpdateConnectionRequest) -> Result<Connection> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id)))?;

        self.client.patch(url, &request).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id)))?;

        self.client.delete(url).await
    }
}
