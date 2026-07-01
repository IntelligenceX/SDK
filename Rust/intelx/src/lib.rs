#![warn(missing_docs)]
//! Async Rust SDK for the [Intelligence X](https://intelx.io) search engine and data archive
//! API.
//!
//! This crate is a Rust port of the
//! [Python `intelx` SDK](https://github.com/IntelligenceX/SDK/tree/master/Python), covering
//! intelligent search, phonebook search, file operations, and the Identity Service
//! (leaked-account search and reverse-domain lookup).
//!
//! # Quick start
//!
//! ```no_run
//! # async fn run() -> intelx::Result<()> {
//! let client = intelx::IntelXClient::new("00000000-0000-0000-0000-000000000000")?;
//! let results = client.search(intelx::SearchParams::new("riseup.net")).await?;
//! for record in &results {
//!     println!("found {} in {}", record.name, record.bucket);
//! }
//! # Ok(())
//! # }
//! ```
//!
//! # Date handling
//!
//! Date fields ([`Item::added`], [`Item::date`]) are kept as plain [`String`]s rather than
//! `chrono::DateTime`, because the server's `YYYY-mm-dd HH:ii:ss` format is not RFC 3339.
//! Parse them yourself if you need typed dates:
//!
//! ```
//! # let date = "2024-01-31 23:59:59".to_string();
//! // requires the `chrono` crate in your own Cargo.toml
//! // let parsed = chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")?;
//! ```

mod client;
mod error;
mod file;
mod identity;
mod models;
mod phonebook;
mod search;
mod util;

pub use client::{IntelXClient, IntelXClientBuilder};
pub use error::{IntelXError, Result, describe_status};
pub use file::{FilePreviewParams, FileReadType};
pub use identity::{IdentityClient, IdentityClientBuilder};
pub use models::{
    AccountRecord, ApiPath, Capabilities, ExportAccountsParams, ExportFormat, IdSearchParams,
    IdSearchResultPage, IdentityRecord, IntelSearchRequest, IntelSearchResultPage,
    IntelSearchStartResponse, Item, PanelSearchResultTag, PhonebookSearchParams,
    PhonebookSearchRequest, PhonebookSearchResultPage, PhonebookSelector, PhonebookTarget,
    Relationship, ReverseDomainParams, ReverseDomainRecord, SearchParams, SearchResult,
    SearchStatus, SortOrder, Tag,
};
pub use phonebook::flatten_selectors;
pub use search::stats;
