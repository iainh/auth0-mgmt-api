use std::fmt;

/// Strongly-typed user identifier.
///
/// Prevents accidental confusion with other ID types (client_id, connection_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Strongly-typed client (application) identifier.
///
/// Prevents accidental confusion with other ID types (user_id, connection_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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

/// Strongly-typed connection identifier.
///
/// Prevents accidental confusion with other ID types (user_id, client_id, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
