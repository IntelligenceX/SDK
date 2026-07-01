use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use super::search_result::SearchResult;

/// Result-sort order for `/intelligent/search` and `/phonebook/search`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SortOrder {
    /// No sorting.
    NoSort = 0,
    /// X-Score ascending: least relevant items first.
    XScoreAsc = 1,
    /// X-Score descending: most relevant items first.
    XScoreDesc = 2,
    /// Date ascending: oldest items first.
    DateAsc = 3,
    /// Date descending: newest items first. The API default.
    #[default]
    DateDesc = 4,
}

/// Export format for `/intelligent/search/export`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum ExportFormat {
    /// CSV summary file.
    Csv = 0,
    /// ZIP archive containing the CSV summary and binary files.
    Zip = 1,
}

/// Result status returned by `/intelligent/search/result` and `/phonebook/search/result`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize_repr, Deserialize_repr)]
#[repr(i32)]
pub enum SearchStatus {
    /// Success with results; the client should keep polling for more.
    Success = 0,
    /// No more results available (this response may still contain results).
    NoMoreResults = 1,
    /// Search ID not found.
    NotFound = 2,
    /// No results yet available; keep trying.
    Pending = 3,
    /// An error occurred.
    Error = 4,
}

/// Request body for `POST /intelligent/search`.
#[derive(Debug, Clone, Serialize)]
pub struct IntelSearchRequest {
    /// The search term. Must be a "strong selector" (email, domain, IP, hash, etc).
    pub term: String,
    /// Buckets to search. Empty means all buckets the account has access to.
    pub buckets: Vec<String>,
    /// Always `0`; reserved by the API.
    pub lookuplevel: i32,
    /// Maximum results to return per bucket.
    pub maxresults: i32,
    /// Timeout in seconds for the search. `0` uses the server default.
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
}

/// Response to `POST /intelligent/search`.
#[derive(Debug, Clone, Deserialize)]
pub struct IntelSearchStartResponse {
    /// The new search's ID, used to fetch results. Absent if `status != 0`.
    pub id: Option<uuid::Uuid>,
    /// `0` = success (id is valid), `1` = invalid term, `2` = max concurrent searches reached.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub status: i32,
    /// Warning that the term resolved to soft (low-quality/generic) selectors.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub softselectorwarning: bool,
}

/// A page of results from `GET /intelligent/search/result`.
#[derive(Debug, Clone, Deserialize)]
pub struct IntelSearchResultPage {
    /// Result records in this page.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub records: Vec<SearchResult>,
    /// Status of the search result; see [`SearchStatus`].
    pub status: SearchStatus,
}

/// Builder-style parameters for [`crate::IntelXClient::search`] and
/// [`crate::IntelXClient::intel_search`], mirroring the Python SDK's
/// `INTEL_SEARCH`/`search()` keyword arguments.
#[derive(Debug, Clone)]
pub struct SearchParams {
    /// The search term. Must be a "strong selector" (email, domain, IP, hash, etc).
    pub term: String,
    /// Maximum results to return per bucket. Defaults to `100`.
    pub maxresults: i32,
    /// Buckets to search. Defaults to empty (all buckets).
    pub buckets: Vec<String>,
    /// Timeout in seconds for the search. Defaults to `5`.
    pub timeout: i32,
    /// Starting date filter, `YYYY-mm-dd HH:ii:ss`. Defaults to empty (unset).
    pub datefrom: String,
    /// Ending date filter, `YYYY-mm-dd HH:ii:ss`. Defaults to empty (unset).
    pub dateto: String,
    /// Sort order for results. Defaults to [`SortOrder::DateDesc`].
    pub sort: SortOrder,
    /// Media type filter. Defaults to `0` (all media types).
    pub media: i32,
    /// IDs of previous searches to terminate before starting this one.
    pub terminate: Vec<uuid::Uuid>,
}

impl SearchParams {
    /// Creates search parameters for `term` with the same defaults as the Python SDK's
    /// `search()`/`INTEL_SEARCH()`: `maxresults=100`, `buckets=[]`, `timeout=5`, `sort=DateDesc`,
    /// `media=0`.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            maxresults: 100,
            buckets: Vec::new(),
            timeout: 5,
            datefrom: String::new(),
            dateto: String::new(),
            sort: SortOrder::DateDesc,
            media: 0,
            terminate: Vec::new(),
        }
    }

    /// Sets the maximum number of results to return per bucket.
    pub fn maxresults(mut self, maxresults: i32) -> Self {
        self.maxresults = maxresults;
        self
    }

    /// Sets the buckets to search.
    pub fn buckets(mut self, buckets: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.buckets = buckets.into_iter().map(Into::into).collect();
        self
    }

    /// Sets the search timeout, in seconds.
    pub fn timeout(mut self, timeout: i32) -> Self {
        self.timeout = timeout;
        self
    }

    /// Sets the starting date filter (`YYYY-mm-dd HH:ii:ss`).
    pub fn datefrom(mut self, datefrom: impl Into<String>) -> Self {
        self.datefrom = datefrom.into();
        self
    }

    /// Sets the ending date filter (`YYYY-mm-dd HH:ii:ss`).
    pub fn dateto(mut self, dateto: impl Into<String>) -> Self {
        self.dateto = dateto.into();
        self
    }

    /// Sets the result sort order.
    pub fn sort(mut self, sort: SortOrder) -> Self {
        self.sort = sort;
        self
    }

    /// Sets the media type filter.
    pub fn media(mut self, media: i32) -> Self {
        self.media = media;
        self
    }

    /// Sets prior search IDs to terminate before starting this search.
    pub fn terminate(mut self, terminate: impl IntoIterator<Item = uuid::Uuid>) -> Self {
        self.terminate = terminate.into_iter().collect();
        self
    }

    pub(crate) fn into_request(self) -> IntelSearchRequest {
        IntelSearchRequest {
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
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intel_search_request_serializes_with_python_compatible_field_names() {
        let req = SearchParams::new("riseup.net").into_request();
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
        ] {
            assert!(obj.contains_key(key), "missing key: {key}");
        }
        assert_eq!(obj["term"], "riseup.net");
        assert_eq!(obj["lookuplevel"], 0);
        assert_eq!(obj["maxresults"], 100);
        assert_eq!(obj["sort"], 4);
    }

    #[test]
    fn search_status_round_trips_via_repr() {
        let json = "0";
        let status: SearchStatus = serde_json::from_str(json).unwrap();
        assert_eq!(status, SearchStatus::Success);
        assert_eq!(serde_json::to_string(&SearchStatus::Pending).unwrap(), "3");
    }

    #[test]
    fn search_params_builder_overrides_defaults() {
        let params = SearchParams::new("test.com")
            .maxresults(50)
            .buckets(["pastes", "darknet.i2p"])
            .sort(SortOrder::XScoreDesc);
        assert_eq!(params.maxresults, 50);
        assert_eq!(params.buckets, vec!["pastes", "darknet.i2p"]);
        assert_eq!(params.sort, SortOrder::XScoreDesc);
    }
}
