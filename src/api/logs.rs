use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::logs::{ListLogsParams, LogEvent, LogsPage};

/// API operations for Auth0 Logs.
///
/// Provides methods to retrieve and search authentication and management API logs.
/// Logs include information about user authentication events, API calls, and system events.
///
/// # Examples
///
/// ```ignore
/// use auth0_mgmt_api::client::ManagementClient;
/// use auth0_mgmt_api::types::logs::ListLogsParams;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ManagementClient::builder("https://example.auth0.com", "token").build()?;>
///
/// // List recent logs
/// let params = ListLogsParams {
///     page: Some(0),
///     per_page: Some(50),
///     sort: Some("date:-1".to_string()),
///     ..Default::default()
/// };
/// let logs = client.logs().list(Some(params)).await?;
///
/// // Get a specific log entry
/// let log = client.logs().get("log_1234567890").await?;
/// println!("Event: {} at {}", log.event_type, log.date.unwrap_or_default());
/// # Ok(())
/// # }
/// ```
///
/// See the [Auth0 Logs API documentation](https://auth0.com/docs/api/management/v2#!/Logs/get_logs)>
/// for detailed information on logs and available operations.
pub struct LogsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> LogsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// List or search log entries with optional pagination, filtering, and searching.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering, searching, sorting, and pagination.
    ///
    /// # Returns
    ///
    /// Returns a vector of log entries matching the criteria.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Get failed login attempts
    /// let params = ListLogsParams {
    ///     q: Some("type:f".to_string()),
    ///     sort: Some("date:-1".to_string()),
    ///     per_page: Some(100),
    ///     ..Default::default()
    /// };
    /// let failed_logins = client.logs().list(Some(params)).await?;
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Logs/get_logs>
    pub async fn list(&self, params: Option<ListLogsParams>) -> Result<Vec<LogEvent>> {
        let mut url = self.client.base_url().join("api/v2/logs")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    /// List or search log entries with pagination totals.
    ///
    /// This method automatically sets `include_totals` to `true` and returns
    /// a paginated response with total count information.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering, searching, sorting, and pagination.
    ///
    /// # Returns
    ///
    /// Returns a paginated response containing log entries and pagination metadata.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Logs/get_logs>
    pub async fn list_with_totals(&self, params: Option<ListLogsParams>) -> Result<LogsPage> {
        let mut url = self.client.base_url().join("api/v2/logs")?;

        let p = params.unwrap_or_default();
        let mut query = serde_urlencoded::to_string(&p)
            .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
        if query.is_empty() {
            query = "include_totals=true".to_string();
        } else {
            query.push_str("&include_totals=true");
        }
        url.set_query(Some(&query));

        self.client.get(url).await
    }

    /// Get a specific log entry by its log ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The log's unique identifier.
    ///
    /// # Returns
    ///
    /// Returns the log entry details if found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let log = client.logs().get("log_abc123").await?;
    /// println!("User: {}, Event: {}", log.user_id.unwrap_or_default(), log.event_type);
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Logs/get_logs_by_id>
    pub async fn get(&self, id: &str) -> Result<LogEvent> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/logs/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }
}
