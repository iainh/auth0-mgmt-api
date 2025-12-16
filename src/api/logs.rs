use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::logs::{ListLogsParams, LogEvent};

pub struct LogsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> LogsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: Option<ListLogsParams>) -> Result<Vec<LogEvent>> {
        let mut url = self.client.base_url().join("api/v2/logs")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    pub async fn get(&self, id: &str) -> Result<LogEvent> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/logs/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }
}
