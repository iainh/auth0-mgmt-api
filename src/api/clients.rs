use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::clients::{Client, CreateClientRequest, ListClientsParams, UpdateClientRequest};
use crate::types::ClientId;

/// API operations for Auth0 Applications (Clients).
///
/// Provides methods to create, read, update, and delete applications. Supports
/// client credential rotation and filtering by application type and ownership.
///
/// # Examples
///
/// ```ignore
/// use auth0_mgmt_api::client::ManagementClient;
/// use auth0_mgmt_api::types::clients::{CreateClientRequest, ListClientsParams};
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// let client = ManagementClient::builder("https://example.auth0.com", "token").build()?;>
///
/// // List applications
/// let params = ListClientsParams {
///     page: Some(0),
///     per_page: Some(10),
///     ..Default::default()
/// };
/// let apps = client.clients().list(Some(params)).await?;
///
/// // Create a new application
/// let new_app = CreateClientRequest {
///     name: "My Web App".to_string(),
///     app_type: Some(auth0_mgmt_api::AppType::RegularWeb),
///     ..Default::default()
/// };
/// let created = client.clients().create(new_app).await?;
///
/// // Rotate client secret
/// let rotated = client.clients().rotate_secret(&created.client_id).await?;
/// println!("New secret: {}", rotated.client_secret.unwrap_or_default());
/// # Ok(())
/// # }
/// ```
///
/// See the [Auth0 Clients API documentation](https://auth0.com/docs/api/management/v2#!/Clients/get_clients)>
/// for detailed information on applications and available operations.
pub struct ClientsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> ClientsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// List or search applications with optional pagination and filtering.
    ///
    /// # Arguments
    ///
    /// * `params` - Optional query parameters for filtering, pagination, and field selection.
    ///
    /// # Returns
    ///
    /// Returns a vector of applications matching the criteria.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/get_clients>
    pub async fn list(&self, params: Option<ListClientsParams>) -> Result<Vec<Client>> {
        let mut url = self.client.base_url().join("api/v2/clients")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    /// Get an application by its client ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The application's client_id.
    ///
    /// # Returns
    ///
    /// Returns the application details if found.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use auth0_mgmt_api::ClientId;
    /// let app = client.clients().get(ClientId::new("YOUR_CLIENT_ID")).await?;
    /// println!("App name: {}", app.name.unwrap_or_default());
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/get_clients_by_id>
    pub async fn get(&self, id: ClientId) -> Result<Client> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id.as_str())))?;

        self.client.get(url).await
    }

    /// Create a new application.
    ///
    /// # Arguments
    ///
    /// * `request` - Application creation parameters including name and optional settings.
    ///
    /// # Returns
    ///
    /// Returns the newly created application with client_id and client_secret.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let new_app = CreateClientRequest {
    ///     name: "My Native App".to_string(),
    ///     app_type: Some(auth0_mgmt_api::AppType::Native),
    ///     callbacks: Some(vec!["myapp://callback".to_string()]),
    ///     ..Default::default()
    /// };
    /// let created = client.clients().create(new_app).await?;
    /// println!("Created app with ID: {}", created.client_id);
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/post_clients>
    pub async fn create(&self, request: CreateClientRequest) -> Result<Client> {
        let url = self.client.base_url().join("api/v2/clients")?;
        self.client.post(url, &request).await
    }

    /// Update an application by its client ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The application's client_id.
    /// * `request` - Application fields to update. Only provided fields are modified.
    ///
    /// # Returns
    ///
    /// Returns the updated application details.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/patch_clients_by_id>
    pub async fn update(&self, id: ClientId, request: UpdateClientRequest) -> Result<Client> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id.as_str())))?;

        self.client.patch(url, &request).await
    }

    /// Delete an application by its client ID.
    ///
    /// # Arguments
    ///
    /// * `id` - The application's client_id.
    ///
    /// # Returns
    ///
    /// Returns success or error.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/delete_clients_by_id>
    pub async fn delete(&self, id: ClientId) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/clients/{}", urlencoding::encode(id.as_str())))?;

        self.client.delete(url).await
    }

    /// Rotate the client secret for an application.
    ///
    /// # Arguments
    ///
    /// * `id` - The application's client_id.
    ///
    /// # Returns
    ///
    /// Returns the updated application with the new client_secret.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// use auth0_mgmt_api::ClientId;
    /// let rotated = client.clients().rotate_secret(ClientId::new("YOUR_CLIENT_ID")).await?;
    /// println!("New secret: {}", rotated.client_secret.unwrap_or_default());
    /// ```
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2#!/Clients/post_rotate_secret>
    pub async fn rotate_secret(&self, id: ClientId) -> Result<Client> {
        let url = self.client.base_url().join(&format!(
            "api/v2/clients/{}/rotate-secret",
            urlencoding::encode(id.as_str())
        ))?;

        self.client.post(url, &()).await
    }
}
