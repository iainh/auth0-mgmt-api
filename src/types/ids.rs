use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Strongly-typed user identifier.
///
/// Prevents accidental confusion with other ID types (client_id, connection_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(String);

impl UserId {
    /// Create a new user ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the user ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for UserId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for UserId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for UserId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for UserId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Strongly-typed client (application) identifier.
///
/// Prevents accidental confusion with other ID types (user_id, connection_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientId(String);

impl ClientId {
    /// Create a new client ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the client ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ClientId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ClientId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for ClientId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ClientId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Strongly-typed connection identifier.
///
/// Prevents accidental confusion with other ID types (user_id, client_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ConnectionId(String);

impl ConnectionId {
    /// Create a new connection ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the connection ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ConnectionId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ConnectionId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ConnectionId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for ConnectionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ConnectionId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Strongly-typed asynchronous job identifier.
///
/// Prevents accidental confusion with user, client, and connection identifiers.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct JobId(String);

impl JobId {
    /// Create a new job ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the job ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for JobId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for JobId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for JobId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for JobId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for JobId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}

/// Strongly-typed client credential identifier.
///
/// Prevents accidental confusion with the client that owns the credential.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ClientCredentialId(String);

impl ClientCredentialId {
    /// Create a new client credential ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Get the client credential ID as a string.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// Convert into the inner string.
    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Display for ClientCredentialId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ClientCredentialId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for ClientCredentialId {
    fn from(id: &str) -> Self {
        Self(id.to_string())
    }
}

impl AsRef<str> for ClientCredentialId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl Deref for ClientCredentialId {
    type Target = str;

    fn deref(&self) -> &str {
        &self.0
    }
}
