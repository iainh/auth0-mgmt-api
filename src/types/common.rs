use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_totals: Option<bool>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PaginatedResponse<T> {
    #[serde(flatten)]
    pub items: Vec<T>,
    pub start: Option<u32>,
    pub limit: Option<u32>,
    pub total: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata(pub serde_json::Map<String, serde_json::Value>);

impl Default for Metadata {
    fn default() -> Self {
        Self(serde_json::Map::new())
    }
}
