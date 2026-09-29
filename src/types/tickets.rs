use serde::{Deserialize, Serialize};

/// Request payload for a password-change ticket identified by email and database connection.
///
/// See the [Auth0 Create a Password Change Ticket documentation](https://auth0.com/docs/api/management/v2/tickets/post-password-change).
#[derive(Debug, Clone, Serialize)]
pub struct PasswordChangeTicketRequest {
    pub email: String,
    pub connection_id: String,
}

/// A newly created password-change ticket.
#[derive(Debug, Clone, Deserialize)]
pub struct PasswordChangeTicketResponse {
    pub ticket: String,
}
