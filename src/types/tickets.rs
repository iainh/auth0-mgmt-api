use serde::{Deserialize, Serialize};

use super::{ClientId, ConnectionId, UserId};

/// The user a password-change ticket is created for.
///
/// Auth0 identifies the user either by `user_id` or by `email` within a
/// database connection. The two forms cannot be combined.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum PasswordChangeTicketTarget {
    /// Identify the user by ID, optionally narrowing to one linked identity.
    User {
        user_id: UserId,
        #[serde(skip_serializing_if = "Option::is_none")]
        identity: Option<PasswordChangeTicketIdentity>,
    },
    /// Identify the user by email address within a database connection.
    Email {
        email: String,
        connection_id: ConnectionId,
    },
}

/// A specific identity of the user to change the password for.
///
/// Auth0 marks this as Early Access and only supports the `auth0` provider.
/// `user_id` is the identity's provider-specific ID, not the full [`UserId`].
#[derive(Debug, Clone, Serialize)]
pub struct PasswordChangeTicketIdentity {
    pub user_id: String,
    pub provider: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connection_id: Option<ConnectionId>,
}

impl PasswordChangeTicketIdentity {
    /// Identify an `auth0` database identity by its provider-specific ID.
    pub fn auth0(user_id: impl Into<String>) -> Self {
        Self {
            user_id: user_id.into(),
            provider: "auth0".to_owned(),
            connection_id: None,
        }
    }
}

/// Request payload for creating a password-change ticket.
///
/// Start from [`Self::for_user`] or [`Self::for_email`] and set optional
/// fields with struct update syntax:
///
/// ```
/// use auth0_mgmt_api::{PasswordChangeTicketRequest, UserId};
///
/// let request = PasswordChangeTicketRequest {
///     ttl_sec: Some(3600),
///     mark_email_as_verified: Some(true),
///     ..PasswordChangeTicketRequest::for_user(UserId::new("auth0|123"))
/// };
/// ```
///
/// Auth0 rejects `result_url` together with `organization_id`.
///
/// See the [Auth0 Create a Password Change Ticket documentation](https://auth0.com/docs/api/management/v2/tickets/post-password-change).
#[derive(Debug, Clone, Serialize)]
pub struct PasswordChangeTicketRequest {
    #[serde(flatten)]
    pub target: PasswordChangeTicketTarget,
    /// URL to redirect to after the ticket is used (classic Universal Login).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result_url: Option<String>,
    /// Application whose details and login route are used for the flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_id: Option<ClientId>,
    /// Organization whose branding and parameters are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organization_id: Option<String>,
    /// Ticket lifetime in seconds. Auth0 uses 5 days when unset or `0`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ttl_sec: Option<u64>,
    /// Whether to set `email_verified` to `true` when the ticket is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mark_email_as_verified: Option<bool>,
    /// Whether to include the email address in the redirect URL.
    #[serde(
        rename = "includeEmailInRedirect",
        skip_serializing_if = "Option::is_none"
    )]
    pub include_email_in_redirect: Option<bool>,
}

impl PasswordChangeTicketRequest {
    /// Create a request for a user identified by ID.
    pub fn for_user(user_id: impl Into<UserId>) -> Self {
        Self::new(PasswordChangeTicketTarget::User {
            user_id: user_id.into(),
            identity: None,
        })
    }

    /// Create a request for a user identified by email within a database connection.
    pub fn for_email(email: impl Into<String>, connection_id: impl Into<ConnectionId>) -> Self {
        Self::new(PasswordChangeTicketTarget::Email {
            email: email.into(),
            connection_id: connection_id.into(),
        })
    }

    fn new(target: PasswordChangeTicketTarget) -> Self {
        Self {
            target,
            result_url: None,
            client_id: None,
            organization_id: None,
            ttl_sec: None,
            mark_email_as_verified: None,
            include_email_in_redirect: None,
        }
    }
}

/// A newly created password-change ticket.
#[derive(Debug, Clone, Deserialize)]
pub struct PasswordChangeTicketResponse {
    /// URL the user opens to change their password.
    pub ticket: String,
}
