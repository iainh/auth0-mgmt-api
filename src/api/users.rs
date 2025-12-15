use crate::client::ManagementClient;
use crate::error::{Auth0Error, Result};
use crate::types::users::{CreateUserRequest, ListUsersParams, UpdateUserRequest, User};

pub struct UsersApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> UsersApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: Option<ListUsersParams>) -> Result<Vec<User>> {
        let mut url = self.client.base_url().join("api/v2/users")?;

        if let Some(p) = params {
            let query = serde_urlencoded::to_string(&p)
                .map_err(|e| Auth0Error::Configuration(e.to_string()))?;
            url.set_query(Some(&query));
        }

        self.client.get(url).await
    }

    pub async fn get(&self, id: &str) -> Result<User> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.get(url).await
    }

    pub async fn create(&self, request: CreateUserRequest) -> Result<User> {
        let url = self.client.base_url().join("api/v2/users")?;
        self.client.post(url, &request).await
    }

    pub async fn update(&self, id: &str, request: UpdateUserRequest) -> Result<User> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.patch(url, &request).await
    }

    pub async fn delete(&self, id: &str) -> Result<()> {
        let url = self
            .client
            .base_url()
            .join(&format!("api/v2/users/{}", urlencoding::encode(id)))?;

        self.client.delete(url).await
    }

    pub async fn get_by_email(&self, email: &str) -> Result<Vec<User>> {
        let mut url = self.client.base_url().join("api/v2/users-by-email")?;
        url.query_pairs_mut().append_pair("email", email);
        self.client.get(url).await
    }
}
