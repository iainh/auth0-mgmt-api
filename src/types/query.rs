use serde::{Deserialize, Serialize};

/// Direction for sorting query results.
///
/// Used in conjunction with SortSpec to specify sort order.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    /// Sort in ascending order (A-Z, 0-9, oldest first)
    #[serde(rename = "1")]
    Ascending,
    /// Sort in descending order (Z-A, 9-0, newest first)
    #[serde(rename = "-1")]
    Descending,
}

impl SortDirection {
    /// Convert to Auth0 API format string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SortDirection::Ascending => "1",
            SortDirection::Descending => "-1",
        }
    }
}

/// Specification for sorting query results.
///
/// Combines a field name with sort direction.
///
/// # Examples
///
/// ```ignore
/// let sort = SortSpec::new("created_at", SortDirection::Descending);
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SortSpec {
    field: String,
    direction: SortDirection,
}

impl SortSpec {
    /// Create a new sort specification.
    pub fn new(field: impl Into<String>, direction: SortDirection) -> Self {
        Self {
            field: field.into(),
            direction,
        }
    }

    /// Create a sort specification with ascending order.
    pub fn ascending(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Ascending)
    }

    /// Create a sort specification with descending order.
    pub fn descending(field: impl Into<String>) -> Self {
        Self::new(field, SortDirection::Descending)
    }

    /// Get the field name.
    pub fn field(&self) -> &str {
        &self.field
    }

    /// Get the sort direction.
    pub fn direction(&self) -> SortDirection {
        self.direction
    }

    /// Convert to Auth0 API format string (e.g., "created_at:-1").
    pub fn to_api_string(&self) -> String {
        format!("{}:{}", self.field, self.direction.as_str())
    }
}

impl std::fmt::Display for SortSpec {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_api_string())
    }
}

/// Validated page number for pagination.
///
/// Pages are 0-indexed. Only allows values >= 0.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub struct Page(u32);

impl Page {
    /// Create a new page number.
    ///
    /// # Errors
    ///
    /// This function will not error; all u32 values are valid page numbers.
    /// Pages are 0-indexed, so page 0 is the first page.
    pub fn new(number: u32) -> Self {
        Self(number)
    }

    /// Get the page number.
    pub fn number(&self) -> u32 {
        self.0
    }
}

impl From<u32> for Page {
    fn from(number: u32) -> Self {
        Self::new(number)
    }
}

impl std::fmt::Display for Page {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Validated number of items per page.
///
/// Must be > 0. Typical values are 10, 25, 50, 100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerPage(u32);

impl PerPage {
    /// Create a new per-page count.
    ///
    /// # Errors
    ///
    /// Returns an error if count is 0.
    pub fn new(count: u32) -> Result<Self, &'static str> {
        if count == 0 {
            Err("per_page must be greater than 0")
        } else {
            Ok(Self(count))
        }
    }

    /// Get the per-page count.
    pub fn count(&self) -> u32 {
        self.0
    }
}

impl TryFrom<u32> for PerPage {
    type Error = &'static str;

    fn try_from(count: u32) -> Result<Self, Self::Error> {
        Self::new(count)
    }
}

impl std::fmt::Display for PerPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Search engine version for user search queries.
///
/// Auth0 supports both v1 (legacy) and v3 (current) search engines.
/// v3 is recommended for new implementations.
///
/// See the [Auth0 User Search documentation](https://auth0.com/docs/api/management/v2/#!/Users/get_users)
/// for differences between versions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum SearchEngine {
    /// Legacy search engine (v1)
    #[serde(rename = "v1")]
    V1,
    /// Current search engine (v3) - recommended
    #[serde(rename = "v3")]
    #[default]
    V3,
}

impl SearchEngine {
    /// Get the search engine version as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            SearchEngine::V1 => "v1",
            SearchEngine::V3 => "v3",
        }
    }
}

impl std::fmt::Display for SearchEngine {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_spec() {
        let sort = SortSpec::descending("created_at");
        assert_eq!(sort.to_api_string(), "created_at:-1");

        let sort = SortSpec::ascending("name");
        assert_eq!(sort.to_api_string(), "name:1");
    }

    #[test]
    fn test_per_page_validation() {
        assert!(PerPage::new(0).is_err());
        assert!(PerPage::new(1).is_ok());
        assert!(PerPage::new(100).is_ok());
    }

    #[test]
    fn test_page_default() {
        assert_eq!(Page::default().number(), 0);
    }

    #[test]
    fn test_search_engine() {
        assert_eq!(SearchEngine::V1.as_str(), "v1");
        assert_eq!(SearchEngine::V3.as_str(), "v3");
        assert_eq!(SearchEngine::default(), SearchEngine::V3);
    }
}
