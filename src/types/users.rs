use serde::{Deserialize, Serialize};

use super::Metadata;

/// Represents an Auth0 user.
///
/// See the [Auth0 User Profile documentation](https://auth0.com/docs/users/manage-users#user-profiles)
/// for detailed information about user properties.
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub user_id: String,
    pub email: Option<String>,
    pub email_verified: Option<bool>,
    pub username: Option<String>,
    pub phone_number: Option<String>,
    pub phone_verified: Option<bool>,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub identities: Option<Vec<Identity>>,
    pub app_metadata: Option<Metadata>,
    pub user_metadata: Option<Metadata>,
    pub picture: Option<String>,
    pub name: Option<String>,
    pub nickname: Option<String>,
    pub given_name: Option<String>,
    pub family_name: Option<String>,
    pub blocked: Option<bool>,
    pub last_ip: Option<String>,
    pub last_login: Option<String>,
    pub logins_count: Option<u64>,
}

/// Represents a user's identity (connection to an auth provider).
///
/// Each user can have multiple identities linked to different providers or connections.
#[derive(Debug, Clone, Deserialize)]
pub struct Identity {
    pub connection: String,
    pub user_id: String,
    pub provider: String,
    #[serde(rename = "isSocial")]
    pub is_social: bool,
}

/// Request payload for creating a new user.
///
/// # Examples
///
/// ```ignore
/// let user = CreateUserRequest {
///     connection: "Username-Password-Authentication".to_string(),
///     email: Some("user@example.com".to_string()),
///     password: Some("SecurePassword123!".to_string()),
///     name: Some("John Doe".to_string()),
///     ..Default::default()
/// };
/// ```
///
/// See the [Auth0 Create User documentation](https://auth0.com/docs/api/management/v2#!/Users/post_users)
/// for detailed information about user creation.
#[derive(Debug, Clone, Default, Serialize)]
pub struct CreateUserRequest {
    pub connection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub given_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_email: Option<bool>,
}

/// Request payload for updating a user.
///
/// # Examples
///
/// ```ignore
/// let update = UpdateUserRequest {
///     email: Some("newemail@example.com".to_string()),
///     email_verified: Some(true),
///     blocked: Some(false),
///     ..Default::default()
/// };
/// ```
///
/// See the [Auth0 Update User documentation](https://auth0.com/docs/api/management/v2#!/Users/patch_users_by_id)
/// for detailed information about user updates.
#[derive(Debug, Clone, Default, Serialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_verified: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_metadata: Option<Metadata>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_metadata: Option<Metadata>,
    #[serde()]
    pub given_name: Option<String>,
    #[serde()]
    pub family_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_email: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verify_phone_number: Option<bool>,
}

/// Query parameters for listing users.
///
/// See the [Auth0 List Users documentation](https://auth0.com/docs/api/management/v2#!/Users/get_users)
/// for detailed information about available filters and search options.
#[derive(Debug, Clone, Default, Serialize)]
pub struct ListUsersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fields: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_fields: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub q: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub search_engine: Option<String>,
}

/// Query parameters for getting user logs.
///
/// See the [Auth0 Get User Logs documentation](https://auth0.com/docs/api/management/v2/users/get-logs-by-user)
/// for detailed information about available options.
#[derive(Debug, Clone, Default, Serialize)]
pub struct GetUserLogsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort: Option<String>,
}

/// Paginated response for user list operations.
///
/// Returned when `include_totals` is set to `true` in list parameters.
#[derive(Debug, Clone, Deserialize)]
pub struct UsersPage {
    /// List of users in this page.
    pub users: Vec<User>,
    /// Starting index of this page (zero-based).
    pub start: u32,
    /// Maximum number of results per page.
    pub limit: u32,
    /// Total number of users matching the query.
    pub total: u32,
}
