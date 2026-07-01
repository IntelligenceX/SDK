use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};

use super::item::Item;

/// An identity-leak result record from `/live/search/result`.
///
/// The Identity Service's record shape is not described by the public OpenAPI schemas the way
/// `SearchResult` is, and the Python SDK never validates it either. This type captures the
/// fields known to be used by the existing Python CLI/examples and preserves any other fields
/// under `extra` so callers can still reach them via [`IdentityRecord::extra`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IdentityRecord {
    /// The underlying item meta-data, when present.
    #[serde(default)]
    pub item: Option<Item>,
    /// Any fields not modeled above, preserved verbatim.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A page of results from `GET /live/search/result`.
#[derive(Debug, Clone, Deserialize)]
pub struct IdSearchResultPage {
    /// `0`/`1` = results available (keep polling), `2` = terminated, `3` = search id not found.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub status: i32,
    /// Result records in this page.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub records: Vec<IdentityRecord>,
}

/// A leaked-account record from `/accounts/csv`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountRecord {
    /// The leaked username/email.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub user: String,
    /// The leaked password (may be hashed, depending on the source).
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub password: String,
    /// Describes how `password` is encoded (plaintext, hash algorithm, etc).
    #[serde(default)]
    pub passwordtype: Value,
    /// Short name of the breach source.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub sourceshort: String,
    /// Any fields not modeled above, preserved verbatim.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// A reverse-domain-lookup record from `/reverse/domain`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReverseDomainRecord {
    /// The leaked username/email.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub user: String,
    /// The leaked password.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub password: String,
    /// The URL the credentials were associated with.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub url: String,
    /// Short name of the breach source.
    #[serde(default, deserialize_with = "crate::util::null_as_default")]
    pub sourceshort: String,
    /// Any fields not modeled above, preserved verbatim.
    #[serde(flatten)]
    pub extra: Map<String, Value>,
}

/// Builder-style parameters for [`crate::IdentityClient::idsearch`], mirroring the Python SDK's
/// `IdentityService.idsearch()` keyword arguments.
#[derive(Debug, Clone)]
pub struct IdSearchParams {
    /// The search term: an email address, domain, SSN, or credit card number.
    pub term: String,
    /// Maximum results to return. Defaults to `100`.
    pub maxresults: i32,
    /// Optional single bucket filter. Defaults to empty (all buckets).
    pub bucket: String,
    /// Timeout in seconds. Defaults to `5`.
    pub timeout: i32,
    /// Starting date filter, `YYYY-mm-dd HH:ii:ss`. Defaults to empty.
    pub datefrom: String,
    /// Ending date filter, `YYYY-mm-dd HH:ii:ss`. Defaults to empty.
    pub dateto: String,
    /// Prior search IDs to terminate first.
    pub terminate: Vec<uuid::Uuid>,
    /// Whether the server should run additional analysis on results. Defaults to `false`.
    pub analyze: bool,
    /// Whether to skip invalid entries server-side (recommended). Defaults to `false`.
    pub skip_invalid: bool,
}

impl IdSearchParams {
    /// Creates identity search parameters for `term` with Python-compatible defaults.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            maxresults: 100,
            bucket: String::new(),
            timeout: 5,
            datefrom: String::new(),
            dateto: String::new(),
            terminate: Vec::new(),
            analyze: false,
            skip_invalid: false,
        }
    }

    /// Sets the maximum number of results to return.
    pub fn maxresults(mut self, maxresults: i32) -> Self {
        self.maxresults = maxresults;
        self
    }

    /// Sets the single-bucket filter.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.bucket = bucket.into();
        self
    }

    /// Sets whether to skip invalid entries server-side.
    pub fn skip_invalid(mut self, skip_invalid: bool) -> Self {
        self.skip_invalid = skip_invalid;
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
}

/// Builder-style parameters for [`crate::IdentityClient::export_accounts`], mirroring the
/// Python SDK's `export_accounts()` keyword arguments.
#[derive(Debug, Clone)]
pub struct ExportAccountsParams {
    /// The search term: a domain or email address.
    pub term: String,
    /// Maximum results to return. Defaults to `10`.
    pub maxresults: i32,
    /// Optional single bucket filter. Defaults to empty.
    pub bucket: String,
    /// Starting date filter. Defaults to empty.
    pub datefrom: String,
    /// Ending date filter. Defaults to empty.
    pub dateto: String,
    /// Prior search IDs to terminate first.
    pub terminate: Vec<uuid::Uuid>,
}

impl ExportAccountsParams {
    /// Creates export-accounts parameters for `term` with Python-compatible defaults.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            maxresults: 10,
            bucket: String::new(),
            datefrom: String::new(),
            dateto: String::new(),
            terminate: Vec::new(),
        }
    }

    /// Sets the maximum number of results to return.
    pub fn maxresults(mut self, maxresults: i32) -> Self {
        self.maxresults = maxresults;
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
}

/// Builder-style parameters for [`crate::IdentityClient::reverse_domain`], mirroring the
/// Python SDK's `reverse_domain()` keyword arguments.
#[derive(Debug, Clone)]
pub struct ReverseDomainParams {
    /// The domain to look up.
    pub term: String,
    /// Maximum results to return. Defaults to `10`.
    pub maxresults: i32,
    /// Starting date filter. Defaults to empty.
    pub datefrom: String,
    /// Ending date filter. Defaults to empty.
    pub dateto: String,
    /// Prior search IDs to terminate first.
    pub terminate: Vec<uuid::Uuid>,
}

impl ReverseDomainParams {
    /// Creates reverse-domain parameters for `term` with Python-compatible defaults.
    pub fn new(term: impl Into<String>) -> Self {
        Self {
            term: term.into(),
            maxresults: 10,
            datefrom: String::new(),
            dateto: String::new(),
            terminate: Vec::new(),
        }
    }

    /// Sets the maximum number of results to return.
    pub fn maxresults(mut self, maxresults: i32) -> Self {
        self.maxresults = maxresults;
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
}
