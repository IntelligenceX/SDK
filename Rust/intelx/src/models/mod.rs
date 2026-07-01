//! Serde data models for Intelligence X API requests and responses.
//!
//! See the crate-level docs for a note on why date fields (`added`, `date`) are kept as
//! [`String`] rather than `chrono::DateTime`: the server's `YYYY-mm-dd HH:ii:ss` format is not
//! RFC 3339, and guessing a deserializer risks silently corrupting dates. Parse them yourself
//! with `chrono::NaiveDateTime::parse_from_str(&item.date, "%Y-%m-%d %H:%M:%S")` if needed.

mod capabilities;
mod identity;
mod item;
mod phonebook;
mod search;
mod search_result;

pub use capabilities::{ApiPath, Capabilities};
pub use identity::{
    AccountRecord, ExportAccountsParams, IdSearchParams, IdSearchResultPage, IdentityRecord,
    ReverseDomainParams, ReverseDomainRecord,
};
pub use item::{Item, Relationship, Tag};
pub use phonebook::{
    PhonebookSearchParams, PhonebookSearchRequest, PhonebookSearchResultPage, PhonebookSelector,
    PhonebookTarget,
};
pub use search::{
    ExportFormat, IntelSearchRequest, IntelSearchResultPage, IntelSearchStartResponse,
    SearchParams, SearchStatus, SortOrder,
};
pub use search_result::{PanelSearchResultTag, SearchResult};
