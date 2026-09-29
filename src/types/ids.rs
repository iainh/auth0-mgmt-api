use serde::{Deserialize, Serialize};
use std::fmt;
use std::ops::Deref;

/// Define a strongly-typed string identifier.
///
/// Each identifier serializes as its inner string and converts from `String`
/// and `&str`, so it can be used directly in request and response types.
macro_rules! define_id {
    ($(#[$meta:meta])* $name:ident, $label:literal) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
        #[serde(transparent)]
        pub struct $name(String);

        impl $name {
            #[doc = concat!("Create a new ", $label, ".")]
            pub fn new(id: impl Into<String>) -> Self {
                Self(id.into())
            }

            #[doc = concat!("Get the ", $label, " as a string.")]
            pub fn as_str(&self) -> &str {
                &self.0
            }

            /// Convert into the inner string.
            pub fn into_inner(self) -> String {
                self.0
            }
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }

        impl From<String> for $name {
            fn from(id: String) -> Self {
                Self(id)
            }
        }

        impl From<&str> for $name {
            fn from(id: &str) -> Self {
                Self(id.to_string())
            }
        }

        impl AsRef<str> for $name {
            fn as_ref(&self) -> &str {
                &self.0
            }
        }

        impl Deref for $name {
            type Target = str;

            fn deref(&self) -> &str {
                &self.0
            }
        }

        impl PartialEq<str> for $name {
            fn eq(&self, other: &str) -> bool {
                self.0 == other
            }
        }

        impl PartialEq<&str> for $name {
            fn eq(&self, other: &&str) -> bool {
                self.0 == *other
            }
        }
    };
}

define_id!(
    /// Strongly-typed user identifier.
    ///
    /// Prevents accidental confusion with other ID types (client_id, connection_id, etc.).
    UserId,
    "user ID"
);

define_id!(
    /// Strongly-typed client (application) identifier.
    ///
    /// Prevents accidental confusion with other ID types (user_id, connection_id, etc.).
    ClientId,
    "client ID"
);

define_id!(
    /// Strongly-typed connection identifier.
    ///
    /// Prevents accidental confusion with other ID types (user_id, client_id, etc.).
    ConnectionId,
    "connection ID"
);

define_id!(
    /// Strongly-typed asynchronous job identifier.
    ///
    /// Prevents accidental confusion with user, client, and connection identifiers.
    JobId,
    "job ID"
);

define_id!(
    /// Strongly-typed client credential identifier.
    ///
    /// Prevents accidental confusion with the client that owns the credential.
    ClientCredentialId,
    "client credential ID"
);
