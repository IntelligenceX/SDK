use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::util::null_as_default;

/// Per-endpoint credit information, as returned by `/authenticate/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPath {
    /// The API path this entry describes.
    #[serde(rename = "Path")]
    pub path: String,
    /// Credits remaining.
    #[serde(rename = "Credit")]
    pub credit: i64,
    /// Maximum credits for this path.
    #[serde(rename = "CreditMax")]
    pub credit_max: i64,
    /// Seconds until the credit allowance resets.
    #[serde(rename = "CreditReset")]
    pub credit_reset: i64,
}

/// The current user's API capabilities, returned by `GET /authenticate/info`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capabilities {
    /// When this capability snapshot was generated.
    #[serde(default, deserialize_with = "null_as_default")]
    pub added: String,
    /// Bucket identifiers the user may search.
    #[serde(default, deserialize_with = "null_as_default")]
    pub buckets: Vec<String>,
    /// Human-readable bucket names, parallel to `buckets`.
    #[serde(default, deserialize_with = "null_as_default")]
    pub bucketsh: Vec<String>,
    /// Per-endpoint credit information, keyed by API path.
    #[serde(default, deserialize_with = "null_as_default")]
    pub paths: HashMap<String, ApiPath>,
    /// Number of currently active searches.
    #[serde(default, deserialize_with = "null_as_default")]
    pub searchesactive: i32,
    /// Maximum number of concurrent searches allowed.
    #[serde(default, deserialize_with = "null_as_default")]
    pub maxconcurrentsearches: i32,
}
