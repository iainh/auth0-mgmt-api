use crate::client::ManagementClient;
use crate::error::Result;
use crate::types::tickets::{PasswordChangeTicketRequest, PasswordChangeTicketResponse};

/// API operations for Auth0 tickets.
pub struct TicketsApi<'a> {
    client: &'a ManagementClient,
}

impl<'a> TicketsApi<'a> {
    pub(crate) fn new(client: &'a ManagementClient) -> Self {
        Self { client }
    }

    /// Create a password-change ticket.
    ///
    /// Requires the `create:user_tickets` scope.
    ///
    /// # Documentation
    ///
    /// <https://auth0.com/docs/api/management/v2/tickets/post-password-change>
    pub async fn create_password_change(
        &self,
        request: PasswordChangeTicketRequest,
    ) -> Result<PasswordChangeTicketResponse> {
        let url = self
            .client
            .base_url()
            .join("api/v2/tickets/password-change")?;
        self.client.post(url, &request).await
    }
}
