use serde::{Deserialize, Serialize};

/// Represents an Auth0 application (client).
///
/// Applications are used to represent the applications and services that need to integrate
/// with your Auth0 tenant.
///
/// See the [Auth0 Application documentation](https://auth0.com/docs/applications)
/// for detailed information about applications.
#[derive(Debug, Clone, Deserialize)]
pub struct Client {
    pub client_id: String,
    pub tenant: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub global: Option<bool>,
    pub client_secret: Option<String>,
    pub app_type: Option<String>,
    pub logo_uri: Option<String>,
    pub is_first_party: Option<bool>,
    pub oidc_conformant: Option<bool>,
    pub callbacks: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub web_origins: Option<Vec<String>>,
    pub client_aliases: Option<Vec<String>>,
    pub allowed_clients: Option<Vec<String>>,
    pub allowed_logout_urls: Option<Vec<String>>,
    pub grant_types: Option<Vec<String>>,
    pub token_endpoint_auth_method: Option<String>,
    pub sso: Option<bool>,
    pub sso_disabled: Option<bool>,
    pub cross_origin_auth: Option<bool>,
    pub cross_origin_loc: Option<String>,
    pub custom_login_page_on: Option<bool>,
    pub custom_login_page: Option<String>,
    pub custom_login_page_preview: Option<String>,
    pub form_template: Option<String>,
    pub is_heroku_app: Option<bool>,
    pub initiate_login_uri: Option<String>,
    pub organization_usage: Option<String>,
    pub organization_require_behavior: Option<String>,
}

/// Request payload for creating a new application.
///
/// # Examples
///
/// ```ignore
/// let app = CreateClientRequest {
///     name: "My Web App".to_string(),
///     app_type: Some("regular_web".to_string()),
///     callbacks: Some(vec!["https://example.com/callback".to_string()]),
///     allowed_logout_urls: Some(vec!["https://example.com/logout".to_string()]),
///     ..Default::default()
/// };
/// ```
///
/// See the [Auth0 Create Application documentation](https://auth0.com/docs/api/management/v2#!/Clients/post_clients)
/// for detailed information about application creation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateClientRequest {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_origins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_logout_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_conformant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_origin_auth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_origin_loc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_login_page_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_login_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_login_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_require_behavior: Option<String>,
}

/// Request payload for updating an application.
///
/// See the [Auth0 Update Application documentation](https://auth0.com/docs/api/management/v2#!/Clients/patch_clients_by_id)
/// for detailed information about application updates.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateClientRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_origins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub web_origins: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_aliases: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_clients: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_logout_urls: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grant_types: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oidc_conformant: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sso_disabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_origin_auth: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cross_origin_loc: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_login_page_on: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_login_page: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiate_login_uri: Option<String>,
}

/// Query parameters for listing applications.
///
/// See the [Auth0 List Applications documentation](https://auth0.com/docs/api/management/v2#!/Clients/get_clients)
/// for detailed information about available filters.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListClientsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_totals: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
}
