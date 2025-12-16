use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::logs::LogEvent;
use crate::types::users::{
    CreateUserRequest, GetUserLogsParams, ListUsersParams, UpdateUserRequest, User,
};

/// API operations for Auth0 Users.
///
/// Provides methods to create, read, update, and delete users. Includes functionality
/// for searching users by email and listing with pagination and filtering.
///
/// # Examples
///
/// ```ignore
/// use auth0_mgmt_api::client::ManagementClient;
/// use auth0_mgmt_api::types::users::{CreateUserRequest, ListUsersParams};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ManagementClient::builder("https://example.auth0.com", "token").build()?;>
///
/// // List users with pagination
/// let params = ListUsersParams {
///     page: Some(0),
///     per_page: Some(50),
///     ..Default::default()
/// };
/// let users = client.users().list(Some(params)).await?;
///
/// // Get a specific user
/// let user = client.users().get("auth0|123456").await?;
///
/// // Create a new user
/// let new_user = CreateUserRequest {
///     connection: "Username-Password-Authentication".to_string(),
///     email: Some("user@example.com".to_string()),
///     password: Some("SecurePassword123!".to_string()),
///     ..Default::default()
/// };
/// let created = client.users().create(new_user).await?;
///
/// // Get users by email
/// let by_email = client.users().get_by_email("user@example.com").await?;
/// # Ok(())
/// # }
/// ```
///
/// See the [Auth0 Users API documentation](https://auth0.com/docs/api/management/v2#!/Users/get_users)>
/// for detailed information on users and available operations.
pub struct UsersApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> UsersApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// List or search users with optional pagination and filtering.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering, searching, sorting, and pagination.
    ///
    /// # Returns
    ///
    /// Returns a vector of users matching the criteria.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = ListUsersParams {
    ///     page: Some(0),
    ///     per_page: Some(50),
    ///     sort: Some("created_at:-1".to_string()),
    ///     ..Default::default()
    /// };
    /// let users = client.users().list(Some(params)).await?;
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/get_users>
    pub async fn list(&self, params: Option<ListUsersParams>) -> Result<Vec<User>> {
        let mut url = self.client.base_url().join("api/v2/users")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    /// Get a user by their user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier.
    ///
    /// # Returns
    ///
    /// Returns the user details if found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let user = client.users().get("auth0|123456").await?;
    /// println!("User email: {:?}", user.email);
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/get_users_by_id>
    pub async fn get(&self, id: &str) -> Result<User> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }

    /// Create a new user.
    ///
    /// # Arguments
    ///
    /// * `request` - User creation parameters including connection and basic user information.
    ///
    /// # Returns
    ///
    /// Returns the newly created user with assigned user_id.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let new_user = CreateUserRequest {
    ///     connection: "Username-Password-Authentication".to_string(),
    ///     email: Some("newuser@example.com".to_string()),
    ///     password: Some("SecurePassword123!".to_string()),
    ///     user_metadata: Some(serde_json::json!({"plan": "free"}).into()),
    ///     ..Default::default()
    /// };
    /// let created = client.users().create(new_user).await?;
    /// println!("Created user: {}", created.user_id);
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/post_users>
    pub async fn create(&self, request: CreateUserRequest) -> Result<User> {
        let url = self.client.base_url().join("api/v2/users")?;
        self.client.post(url, &request).await
    }

    /// Update a user by their user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier.
    /// * `request` - User fields to update. Only provided fields are modified.
    ///
    /// # Returns
    ///
    /// Returns the updated user details.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let update = UpdateUserRequest {
    ///     email: Some("newemail@example.com".to_string()),
    ///     blocked: Some(false),
    ///     ..Default::default()
    /// };
    /// let updated = client.users().update("auth0|123456", update).await?;
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/patch_users_by_id>
    pub async fn update(&self, id: &str, request: UpdateUserRequest) -> Result<User> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.patch(url, &request).await
    }

    /// Delete a user by their user ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier.
    ///
    /// # Returns
    ///
    /// Returns success or error.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// client.users().delete("auth0|123456").await?;
    /// println!("User deleted successfully");
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/delete_users_by_id>
    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.delete(url).await
    }

    /// Get users by email address.
    ///
    /// # Arguments
    ///
    /// * `email` - The email address to search for.
    ///
    /// # Returns
    ///
    /// Returns a vector of users with the specified email address.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let users = client.users().get_by_email("user@example.com").await?;
    /// for user in users {
    ///     println!("Found user: {}", user.user_id);
    /// }
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Users/get_users_by_email>
    pub async fn get_by_email(&self, email: &str) -> Result<Vec<User>> {
        let mut url = self.client.base_url().join("api/v2/users-by-email")?;
        url.query_pairs_mut().append_pair("email", email);
        self.client.get(url).await
    }

    /// Get log events for a specific user.
    ///
    /// # Arguments
    ///
    /// * `id` - The user's unique identifier.
    /// * `params` - Optional query parameters for pagination and sorting.
    ///
    /// # Returns
    ///
    /// Returns a vector of log entries for the user.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let params = GetUserLogsParams {
    ///     per_page: Some(10),
    ///     sort: Some("date:-1".to_string()),
    ///     ..Default::default()
    /// };
    /// let logs = client.users().get_logs("auth0|123456", Some(params)).await?;
    /// for log in logs {
    ///     println!("Event: {:?} at {:?}", log.log_type, log.date);
    /// }
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/users/get-logs-by-user>
    pub async fn get_logs(
        &self,
        id: &str,
        params: Option<GetUserLogsParams>,
    ) -> Result<Vec<LogEvent>> {
        let mut url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}/logs", urlencoding::encode(id)))?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }
}
