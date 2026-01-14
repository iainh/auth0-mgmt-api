use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::connections::{
    Connection, ConnectionsPage, CreateConnectionRequest, ListConnectionsParams, UpdateConnectionRequest,
};
use crate::types::ConnectionId;

/// API operations for Auth0 Connections.
///
/// Provides methods to create, read, update, and delete connections. Connections
/// define how users authenticate (e.g., databases, social providers, enterprise connections).
///
/// # Examples
///
/// ```ignore
/// use auth0_mgmt_api::client::ManagementClient;
/// use auth0_mgmt_api::types::connections::{CreateConnectionRequest, ListConnectionsParams};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ManagementClient::builder("https://example.auth0.com", "token").build()?;>
///
/// // List all connections
/// let connections = client.connections().list(None).await?;
///
/// // List database connections only
/// let params = ListConnectionsParams {
///     strategy: Some(auth0_mgmt_api::ConnectionStrategy::Auth0Database),
///     ..Default::default()
/// };
/// let db_connections = client.connections().list(Some(params)).await?;
///
/// // Create a new database connection
/// let new_conn = CreateConnectionRequest {
///     name: "My Database".to_string(),
///     strategy: auth0_mgmt_api::ConnectionStrategy::Auth0Database,
///     ..Default::default()
/// };
/// let created = client.connections().create(new_conn).await?;
/// # Ok(())
/// # }
/// ```
///
/// See the [Auth0 Connections API documentation](https://auth0.com/docs/api/management/v2#!/Connections/get_connections)>
/// for detailed information on connections and available operations.
pub struct ConnectionsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> ConnectionsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// List or search connections with optional pagination and filtering.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering by strategy, pagination, and field selection.
    ///
    /// # Returns
    ///
    /// Returns a vector of connections matching the criteria.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/get_connections>
    pub async fn list(&self, params: Option<ListConnectionsParams>) -> Result<Vec<Connection>> {
        let mut url = self.client.base_url().join("api/v2/connections")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    /// List or search connections with pagination totals.
    ///
    /// This method automatically sets `include_totals` to `true` and returns
    /// a paginated response with total count information.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering by strategy, pagination, and field selection.
    ///
    /// # Returns
    ///
    /// Returns a paginated response containing connections and pagination metadata.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/get_connections>
    pub async fn list_with_totals(&self, params: Option<ListConnectionsParams>) -> Result<ConnectionsPage> {
        let mut url = self.client.base_url().join("api/v2/connections")?;

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

    /// Get a connection by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The connection's unique identifier.
    ///
    /// # Returns
    ///
    /// Returns the connection details if found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use auth0_mgmt_api::ConnectionId;
    /// let conn = client.connections().get(ConnectionId::new("con_1234567890")).await?;
    /// println!("Connection: {} ({})", conn.name, conn.strategy);
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/get_connections_by_id>
    pub async fn get(&self, id: ConnectionId) -> Result<Connection> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id.as_str())))?;

        self.client.get(url).await
    }

    /// Create a new connection.
    ///
    /// # Arguments
    ///
    /// * `request` - Connection creation parameters including name and strategy type.
    ///
    /// # Returns
    ///
    /// Returns the newly created connection.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/post_connections>
    pub async fn create(&self, request: CreateConnectionRequest) -> Result<Connection> {
        let url = self.client.base_url().join("api/v2/connections")?;
        self.client.post(url, &request).await
    }

    /// Update a connection by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The connection's unique identifier.
    /// * `request` - Connection fields to update. Only provided fields are modified.
    ///
    /// # Returns
    ///
    /// Returns the updated connection details.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/patch_connections_by_id>
    pub async fn update(&self, id: ConnectionId, request: UpdateConnectionRequest) -> Result<Connection> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id.as_str())))?;

        self.client.patch(url, &request).await
    }

    /// Delete a connection by its ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The connection's unique identifier.
    ///
    /// # Returns
    ///
    /// Returns success or error.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Connections/delete_connections_by_id>
    pub async fn delete(&self, id: ConnectionId) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/connections/{}", urlencoding::encode(id.as_str())))?;

        self.client.delete(url).await
    }
}
