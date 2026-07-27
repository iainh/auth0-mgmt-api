use serde::{Deserialize, Serialize};

use super::Patch;
use super::enums::{
    AppType, GrantType, OrganizationRequireBehavior, OrganizationUsage, TokenAuthMethod,
};

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
    pub app_type: Option<AppType>,
    pub logo_uri: Option<String>,
    pub is_first_party: Option<bool>,
    pub oidc_conformant: Option<bool>,
    pub callbacks: Option<Vec<String>>,
    pub allowed_origins: Option<Vec<String>>,
    pub web_origins: Option<Vec<String>>,
    pub client_aliases: Option<Vec<String>>,
    pub allowed_clients: Option<Vec<String>>,
    pub allowed_logout_urls: Option<Vec<String>>,
    pub grant_types: Option<Vec<GrantType>>,
    pub token_endpoint_auth_method: Option<TokenAuthMethod>,
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
    pub organization_usage: Option<OrganizationUsage>,
    pub organization_require_behavior: Option<OrganizationRequireBehavior>,
}

/// Request payload for creating a new application.
///
/// # Examples
///
/// ```ignore
/// use auth0_mgmt_api::AppType;
/// let app = CreateClientRequest {
///     name: "My Web App".to_string(),
///     app_type: Some(AppType::RegularWeb),
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
    pub grant_types: Option<Vec<GrantType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<TokenAuthMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<AppType>,
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
    pub organization_usage: Option<OrganizationUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_require_behavior: Option<OrganizationRequireBehavior>,
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
    pub grant_types: Option<Vec<GrantType>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_endpoint_auth_method: Option<TokenAuthMethod>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<AppType>,
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
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_global: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_first_party: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<AppType>,
}

/// Algorithm used to verify assertions made with a client credential.
///
/// See the [Auth0 Client Credentials documentation](https://auth0.com/docs/api/management/v2/clients/get-credentials).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ClientCredentialAlgorithm {
    RS256,
    RS384,
    PS256,
}

/// Type of public-key credential configured for a client.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientCredentialType {
    PublicKey,
    CertSubjectDn,
    X509Cert,
}

/// A public-key credential configured for an Auth0 client.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientCredential {
    pub id: Option<String>,
    pub name: Option<String>,
    pub kid: Option<String>,
    pub alg: Option<ClientCredentialAlgorithm>,
    pub credential_type: Option<ClientCredentialType>,
    pub subject_dn: Option<String>,
    pub thumbprint_sha256: Option<String>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub expires_at: Option<String>,
}

/// Request payload for creating a client credential.
///
/// `subject_dn` and `pem` are mutually exclusive. Their applicability depends
/// on `credential_type`; see Auth0's endpoint documentation for the accepted
/// combinations.
///
/// See the [Auth0 Create Client Credential documentation](https://auth0.com/docs/api/management/v2/clients/post-credentials).
#[derive(Debug, Clone, Serialize)]
pub struct CreateClientCredentialRequest {
    pub credential_type: ClientCredentialType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subject_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alg: Option<ClientCredentialAlgorithm>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_expiry_from_cert: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kid: Option<String>,
}

/// Request payload for changing a client credential's expiration.
///
/// Use [`Patch::Null`] to remove the expiration or [`Patch::Value`] to set it.
///
/// See the [Auth0 Update Client Credential documentation](https://auth0.com/docs/api/management/v2/clients/patch-credentials-by-credential-id).
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateClientCredentialRequest {
    #[serde(skip_serializing_if = "Patch::is_unset")]
    pub expires_at: Patch<String>,
}

/// Query parameters for connections enabled for a client.
///
/// `strategy` values are serialized as repeated query parameters as defined by
/// OpenAPI's form/explode array encoding.
#[derive(Debug, Clone, Default)]
pub struct ListClientConnectionsParams {
    pub strategy: Option<Vec<String>>,
    pub from: Option<String>,
    pub take: Option<u32>,
    pub fields: Option<String>,
    pub include_fields: Option<bool>,
}

/// A connection enabled for a client.
///
/// Strategy is intentionally a string because Auth0 supports substantially
/// more strategies here than the connection-creation API.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientConnection {
    pub id: Option<String>,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub strategy: Option<String>,
    pub realms: Option<Vec<String>>,
    pub is_domain_connection: Option<bool>,
    pub show_as_button: Option<bool>,
    pub options: Option<serde_json::Value>,
    pub metadata: Option<serde_json::Value>,
    pub authentication: Option<serde_json::Value>,
    pub connected_accounts: Option<serde_json::Value>,
    pub cross_app_access_requesting_app: Option<serde_json::Value>,
    pub cross_app_access_resource_app: Option<serde_json::Value>,
}

/// Checkpoint-paginated connections enabled for a client.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientConnectionsPage {
    pub connections: Vec<ClientConnection>,
    pub next: Option<String>,
}

/// Paginated response for client list operations.
///
/// Returned when `include_totals` is set to `true` in list parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct ClientsPage {
    /// List of clients in this page.
    pub clients: Vec<Client>,
    /// Starting index of this page (zero-based).
    pub start: u32,
    /// Maximum number of results per page.
    pub limit: u32,
    /// Total number of clients matching the query.
    pub total: u32,
}
