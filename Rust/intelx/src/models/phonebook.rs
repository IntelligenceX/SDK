use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::search::SortOrder;

/// What kind of selector to search for in a phonebook search.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum PhonebookTarget {
    /// All selector types.
    #[default]
    All = 0,
    /// Domains only.
    Domains = 1,
    /// Email addresses only.
    EmailAddresses = 2,
    /// URLs only.
    Urls = 3,
}

/// Request body for `POST /phonebook/search`.
#[derive(Debug, Clone, Serialize)]
pub struct PhonebookSearchRequest {
    /// The search term.
    pub term: String,
    /// Buckets to search. Empty means all buckets the account has access to.
    pub buckets: Vec<String>,
    /// Always `0`; reserved by the API.
    pub lookuplevel: i32,
    /// Maximum results to return.
    pub maxresults: i32,
    /// Timeout in seconds for the search.
    pub timeout: i32,
    /// Starting date filter, `YYYY-mm-dd HH:ii:ss` or empty.
    pub datefrom: String,
    /// Ending date filter, `YYYY-mm-dd HH:ii:ss` or empty.
    pub dateto: String,
    /// Sort order for results.
    pub sort: SortOrder,
    /// Media type filter. `0` means all media types.
    pub media: i32,
    /// IDs of previous searches to terminate before starting this one.
    pub terminate: Vec<uuid::Uuid>,
    /// Which selector types to return.
    pub target: PhonebookTarget,
}

/// A single selector discovered by a phonebook search.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonebookSelector {
    /// Numeric selector type.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub selectortype: i32,
    /// Human-readable selector type.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub selectortypeh: String,
    /// The selector's value (e.g. an email address or domain).
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub selectorvalue: String,
}

/// A page of results from `GET /phonebook/search/result`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhonebookSearchResultPage {
    /// Selectors discovered in this page.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub selectors: Vec<PhonebookSelector>,
    /// Status of the search result.
    pub status: super::search::SearchStatus,
}

/// Builder-style parameters for [`crate::IntelXClient::phonebook_search_all`], mirroring the
/// Python SDK's `phonebooksearch()` keyword arguments.
#[derive(Debug, Clone)]
pub struct PhonebookSearchParams {
    /// The search term.
    pub term: String,
    /// Maximum results to return. Defaults to `1000`, matching Python's `phonebooksearch()`.
    pub maxresults: i32,
    /// Buckets to search. Defaults to empty (all buckets).
    pub buckets: Vec<String>,
    /// Timeout in seconds. Defaults to `5`.
    pub timeout: i32,
    /// Starting date filter. Defaults to empty.
    pub datefrom: String,
    /// Ending date filter. Defaults to empty.
    pub dateto: String,
    /// Sort order. Defaults to [`SortOrder::DateDesc`].
    pub sort: SortOrder,
    /// Media type filter. Defaults to `0`.
    pub media: i32,
    /// Prior search IDs to terminate first.
    pub terminate: Vec<uuid::Uuid>,
    /// Which selector types to return. Defaults to [`PhonebookTarget::All`].
    pub target: PhonebookTarget,
}

impl PhonebookSearchParams {
    /// Creates phonebook search parameters for `term` with Python-compatible defaults.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            maxresults: 1000,
            buckets: Vec::new(),
            timeout: 5,
            datefrom: String::new(),
            dateto: String::new(),
            sort: SortOrder::DateDesc,
            media: 0,
            terminate: Vec::new(),
            target: PhonebookTarget::All,
        }
    }

    /// Sets the maximum number of results to return.
    pub fn maxresults(mut self, maxresults: i32) -> Self {
        self.maxresults = maxresults;
        self
    }

    /// Sets the buckets to search.
    pub fn buckets(mut self, buckets: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.buckets = buckets.into_iter().map(Into::into).collect();
        self
    }

    /// Sets which selector types to return.
    pub fn target(mut self, target: PhonebookTarget) -> Self {
        self.target = target;
        self
    }

    pub(crate) fn into_request(self) -> PhonebookSearchRequest {
        PhonebookSearchRequest {
            term: self.term,
            buckets: self.buckets,
            lookuplevel: 0,
            maxresults: self.maxresults,
            timeout: self.timeout,
            datefrom: self.datefrom,
            dateto: self.dateto,
            sort: self.sort,
            media: self.media,
            terminate: self.terminate,
            target: self.target,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phonebook_search_request_serializes_with_python_compatible_field_names() {
        let req = PhonebookSearchParams::new("info@intelx.io").into_request();
        let value = serde_json::to_value(&req).unwrap();
        let obj = value.as_object().unwrap();
        for key in [
            "term",
            "buckets",
            "lookuplevel",
            "maxresults",
            "timeout",
            "datefrom",
            "dateto",
            "sort",
            "media",
            "terminate",
            "target",
        ] {
            assert!(obj.contains_key(key), "missing key: {key}");
        }
        assert_eq!(obj["maxresults"], 1000);
        assert_eq!(obj["target"], 0);
    }
}
