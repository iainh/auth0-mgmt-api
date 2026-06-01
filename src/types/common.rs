use std::ops::{Deref, DerefMut};

use serde::{Deserialize, Serialize};

/// Common pagination parameters for list operations.
///
/// Used to control pagination in API list endpoints.
#[derive(Debug, Clone, Default, Serialize)]
pub struct PaginationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<u32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub per_page: Option<u32>,
}

/// User metadata as a JSON object.
///
/// Metadata is arbitrary JSON data associated with users. Auth0 supports both app_metadata
/// (managed by the application) and user_metadata (managed by the user).
///
/// See the [Auth0 User Metadata documentation](https://auth0.com/docs/users/manage-users#metadata)
/// for detailed information about metadata usage.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata(pub serde_json::Map<String, serde_json::Value>);

impl Default for Metadata {
    fn default() -> Self {
        Self(serde_json::Map::new())
    }
}

impl Deref for Metadata {
    type Target = serde_json::Map<String, serde_json::Value>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Metadata {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
pub enum Patch<T> {
    #[default]
    Unset,
    Null,
    Value(T),
}

impl<T> Serialize for Patch<T>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Patch::Unset | Patch::Null => serializer.serialize_none(),
            Patch::Value(value) => value.serialize(serializer),
        }
    }
}

impl<'de, T> Deserialize<'de> for Patch<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Use Option<T> to capture the difference between a value and null
        Option::<T>::deserialize(deserializer).map(|opt| match opt {
            Some(val) => Patch::Value(val),
            None => Patch::Null,
        })
    }
}

impl<T> Patch<T> {
    pub fn is_unset(&self) -> bool {
        matches!(self, Patch::Unset)
    }
}
