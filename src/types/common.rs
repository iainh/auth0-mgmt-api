use serde::{Deserialize, Serialize};

/// Common pagination parameters for list operations.
///
/// Used to control pagination in API list endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_totals: Option<bool>,
}

/// Represents a paginated response from a list API endpoint.
///
/// Contains the list of items and pagination metadata.
#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    #[serde(flatten)]
    pub items: Vec<T>,
    pub start: Option<u32>,
    pub limit: Option<u32>,
    pub total: Option<u32>,
}

/// User metadata as a JSON object.
///
/// Metadata is arbitrary JSON data associated with users. Auth0 supports both app_metadata
/// (managed by the application) and user_metadata (managed by the user).
///
/// See the [Auth0 User Metadata documentation](https://auth0.com/docs/users/manage-users#metadata)
/// for detailed information about metadata usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata(pub serde_json::Map<String, serde_json::Value>);

impl Default for Metadata {
    fn default() -> Self {
        Self(serde_json::Map::new())
    }
}
