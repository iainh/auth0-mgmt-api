use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Connection {
    pub id: String,
    pub name: String,
    pub display_name: Option<String>,
    pub strategy: String,
    pub realms: Option<Vec<String>>,
    pub is_domain_connection: Option<bool>,
    pub enabled_clients: Option<Vec<String>>,
    pub metadata: Option<serde_json::Value>,
    pub options: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateConnectionRequest {
    pub name: String,
    pub strategy: String,
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

#[derive(Debug, Clone, Default, Serialize)]
pub struct ListConnectionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_totals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strategy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
}
