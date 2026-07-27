use serde::{Deserialize, Serialize};

use super::enums::ConnectionStrategy;

/// Represents an Auth0 connection.
///
/// Connections are sources of users. You can use different types of connections (databases,
/// social providers, enterprise connections, etc.) to allow users to authenticate.
///
/// See the [Auth0 Connection documentation](https://auth0.com/docs/connections)
/// for detailed information about connections and their types.
#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub strategy: ConnectionStrategy,
    pub realms: Option<Vec<String>>,
    pub is_domain_connection: Option<bool>,
    pub enabled_clients: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
    pub options: Option<serde_json::Value>,
}

/// Request payload for creating a new connection.
///
/// # Examples
///
/// ```ignore
/// let conn = CreateConnectionRequest {
///     name: "My Database".to_string(),
///     strategy: "auth0".to_string(),
///     display_name: Some("My Database Connection".to_string()),
///     ..Default::default()
/// };
/// ```
///
/// See the [Auth0 Create Connection documentation](https://auth0.com/docs/api/management/v2#!/Connections/post_connections)
/// for detailed information about connection creation and available strategies.
#[derive(Debug, Clone, Serialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub strategy: ConnectionStrategy,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_domain_connection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

impl Default for CreateConnectionRequest {
    fn default() -> Self {
        Self {
            name: String::new(),
            strategy: ConnectionStrategy::Auth0Database,
            display_name: None,
            options: None,
            enabled_clients: None,
            realms: None,
            is_domain_connection: None,
            metadata: None,
        }
    }
}

/// Request payload for updating a connection.
///
/// See the [Auth0 Update Connection documentation](https://auth0.com/docs/api/management/v2#!/Connections/patch_connections_by_id)
/// for detailed information about connection updates.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateConnectionRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub realms: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_domain_connection: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
}

/// Query parameters for listing connections.
///
/// See the [Auth0 List Connections documentation](https://auth0.com/docs/api/management/v2#!/Connections/get_connections)
/// for detailed information about available filters.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConnectionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<ConnectionStrategy>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
}

/// Query parameters for listing clients enabled for a connection.
///
/// This endpoint uses checkpoint pagination. Pass the `next` value from a
/// response as `from` to retrieve the following page.
///
/// See the [Auth0 Get Enabled Clients documentation](https://auth0.com/docs/api/management/v2/connections/get-connection-clients)
/// for pagination limits and response details.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConnectionClientsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
}

/// A client for which a connection is enabled.
///
/// See the [Auth0 Get Enabled Clients documentation](https://auth0.com/docs/api/management/v2/connections/get-connection-clients).
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionClient {
    pub client_id: String,
}

/// Checkpoint-paginated clients enabled for a connection.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionClientsPage {
    pub clients: Vec<ConnectionClient>,
    /// Token to pass as `from` when requesting the next page, if one exists.
    pub next: Option<String>,
}

/// Enables or disables a connection for one client.
///
/// Auth0 accepts at most 50 changes in a single request.
///
/// See the [Auth0 Update Enabled Clients documentation](https://auth0.com/docs/api/management/v2/connections/patch-clients).
#[derive(Debug, Clone, Serialize)]
pub struct ConnectionClientUpdate {
    pub client_id: String,
    pub status: bool,
}

/// Paginated response for connection list operations.
///
/// Returned when `include_totals` is set to `true` in list parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct ConnectionsPage {
    /// List of connections in this page.
    pub connections: Vec<Connection>,
    /// Starting index of this page (zero-based).
    pub start: u32,
    /// Maximum number of results per page.
    pub limit: u32,
    /// Total number of connections matching the query.
    pub total: u32,
}
