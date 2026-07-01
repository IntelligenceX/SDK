//! The Identity Service: leaked-account search and reverse-domain lookup against
//! `https://3.intelx.io`.

use std::time::Duration;

use crate::client::{IntelXClient, IntelXClientBuilder};
use crate::error::{IntelXError, Result, api_error_from_status};
use crate::models::{
    AccountRecord, ExportAccountsParams, IdSearchParams, IdSearchResultPage, IdentityRecord,
    ReverseDomainParams, ReverseDomainRecord,
};

const DEFAULT_IDENTITY_BASE_URL: &str = "https://3.intelx.io";

/// An async client for the Intelligence X Identity Service (leaked accounts, reverse-domain
/// lookup), default base URL `https://3.intelx.io`.
///
/// Wraps an [`IntelXClient`] rather than extending it (composition over the Python SDK's
/// `IdentityService(intelx)` inheritance), since this client only exposes a small, disjoint
/// method set.
#[derive(Clone)]
pub struct IdentityClient {
    inner: IntelXClient,
}

/// Builder for [`IdentityClient`].
pub struct IdentityClientBuilder {
    inner: IntelXClientBuilder,
}

impl Default for IdentityClientBuilder {
    fn default() -> Self {
        Self {
            inner: IntelXClientBuilder::default().base_url(DEFAULT_IDENTITY_BASE_URL),
        }
    }
}

impl IdentityClientBuilder {
    /// Sets the API key sent as the `X-Key` header on every request. Required.
    pub fn api_key(mut self, api_key: impl Into<String>) -> Self {
        self.inner = self.inner.api_key(api_key);
        self
    }

    /// Overrides the base URL. Defaults to `https://3.intelx.io`.
    pub fn base_url(mut self, base_url: impl Into<String>) -> Self {
        self.inner = self.inner.base_url(base_url);
        self
    }

    /// Overrides the `User-Agent` header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.inner = self.inner.user_agent(user_agent);
        self
    }

    /// Routes all requests through the given proxy URL.
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> Self {
        self.inner = self.inner.proxy(proxy_url);
        self
    }

    /// Sets the per-request HTTP timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.inner = self.inner.timeout(timeout);
        self
    }

    /// Sets the delay applied before each request to stay within the API's rate limit.
    pub fn rate_limit(mut self, rate_limit: Duration) -> Self {
        self.inner = self.inner.rate_limit(rate_limit);
        self
    }

    /// Builds the [`IdentityClient`].
    pub fn build(self) -> Result<IdentityClient> {
        Ok(IdentityClient {
            inner: self.inner.build()?,
        })
    }
}

impl IdentityClient {
    /// Creates a builder for configuring an identity client.
    pub fn builder() -> IdentityClientBuilder {
        IdentityClientBuilder::default()
    }

    /// Creates an identity client for `api_key` using all other defaults.
    pub fn new(api_key: impl Into<String>) -> Result<Self> {
        Self::builder().api_key(api_key).build()
    }

    /// Fetches a page of results for a previously started identity search.
    ///
    /// Mirrors the Python SDK's `IdentityService.get_search_results()`.
    pub async fn get_search_results(
        &self,
        id: uuid::Uuid,
        format: i32,
        maxresults: i32,
    ) -> Result<IdSearchResultPage> {
        self.inner.rate_limit_sleep().await;
        self.inner
            .get(
                "/live/search/result",
                &[
                    ("id", id.to_string()),
                    ("format", format.to_string()),
                    ("limit", maxresults.to_string()),
                ],
            )
            .await
    }

    /// Runs an identity (leaked-data) search to completion against `/live/search/internal`,
    /// polling until the API reports the search is terminated/not-found or `maxresults` has
    /// been satisfied.
    ///
    /// Mirrors the Python SDK's `idsearch()`.
    pub async fn idsearch(&self, params: IdSearchParams) -> Result<Vec<IdentityRecord>> {
        let mut remaining = params.maxresults;
        let search_id: uuid::Uuid = {
            let response: serde_json::Value = self
                .inner
                .get(
                    "/live/search/internal",
                    &[
                        ("selector", params.term),
                        ("bucket", params.bucket),
                        ("skipinvalid", params.skip_invalid.to_string()),
                        ("limit", remaining.to_string()),
                        ("analyze", params.analyze.to_string()),
                        ("datefrom", params.datefrom),
                        ("dateto", params.dateto),
                    ],
                )
                .await?;
            let id = response
                .get("id")
                .and_then(|v| v.as_str())
                .and_then(|s| s.parse().ok());
            id.ok_or_else(|| IntelXError::Api {
                status: 200,
                message: "missing search id in response".into(),
            })?
        };

        let mut results = Vec::new();
        loop {
            self.inner.rate_limit_sleep().await;
            let page = self.get_search_results(search_id, 1, remaining).await?;
            if page.status == 0 || page.status == 2 {
                remaining -= page.records.len() as i32;
                results.extend(page.records);
            }
            let exhausted = remaining <= 0;
            if page.status == 2 || page.status == 3 || exhausted {
                if exhausted || page.status == 3 {
                    let _ = self.terminate_search(search_id).await;
                }
                break;
            }
        }
        Ok(results)
    }

    /// Terminates a previously started identity search. Mirrors the Python SDK's
    /// `terminate_search()`.
    pub async fn terminate_search(&self, id: uuid::Uuid) -> Result<()> {
        let response = self
            .inner
            .get_response("/live/search/internal", &[("id", id.to_string())])
            .await?;
        if response.status().is_success() || response.status() == reqwest::StatusCode::NO_CONTENT {
            Ok(())
        } else {
            Err(api_error_from_status(response.status()))
        }
    }

    /// Exports leaked accounts for `params.term` (an email address or domain). Mirrors the
    /// Python SDK's `export_accounts()`.
    pub async fn export_accounts(
        &self,
        params: ExportAccountsParams,
    ) -> Result<Vec<AccountRecord>> {
        let mut remaining = params.maxresults;
        let response: serde_json::Value = self
            .inner
            .get(
                "/accounts/csv",
                &[
                    ("selector", params.term),
                    ("bucket", params.bucket),
                    ("limit", remaining.to_string()),
                    ("datefrom", params.datefrom),
                    ("dateto", params.dateto),
                ],
            )
            .await?;
        let search_id: uuid::Uuid = response
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| IntelXError::Api {
                status: 200,
                message: "missing search id in response".into(),
            })?;

        let mut results: Vec<AccountRecord> = Vec::new();
        loop {
            self.inner.rate_limit_sleep().await;
            let page: serde_json::Value = self
                .inner
                .get(
                    "/live/search/result",
                    &[
                        ("id", search_id.to_string()),
                        ("limit", remaining.to_string()),
                    ],
                )
                .await?;
            let status = page.get("status").and_then(|v| v.as_i64()).unwrap_or(0);
            let records: Vec<AccountRecord> = page
                .get("records")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default();

            if status == 0 || status == 2 {
                remaining -= records.len() as i32;
                results.extend(records);
            }
            let exhausted = remaining <= 0;
            if status == 2 || exhausted {
                if exhausted {
                    let _ = self.terminate_search(search_id).await;
                }
                break;
            }
        }
        Ok(results)
    }

    /// Performs a reverse-domain lookup for `params.term`. Mirrors the Python SDK's
    /// `reverse_domain()`.
    pub async fn reverse_domain(
        &self,
        params: ReverseDomainParams,
    ) -> Result<Vec<ReverseDomainRecord>> {
        let mut remaining = params.maxresults;
        let response: serde_json::Value = self
            .inner
            .get(
                "/reverse/domain",
                &[
                    ("selector", params.term),
                    ("limit", remaining.to_string()),
                    ("datefrom", params.datefrom),
                    ("dateto", params.dateto),
                ],
            )
            .await?;
        let search_id: uuid::Uuid = response
            .get("id")
            .and_then(|v| v.as_str())
            .and_then(|s| s.parse().ok())
            .ok_or_else(|| IntelXError::Api {
                status: 200,
                message: "missing search id in response".into(),
            })?;

        let mut results: Vec<ReverseDomainRecord> = Vec::new();
        loop {
            self.inner.rate_limit_sleep().await;
            let page: serde_json::Value = self
                .inner
                .get(
                    "/live/search/result",
                    &[
                        ("id", search_id.to_string()),
                        ("limit", remaining.to_string()),
                    ],
                )
                .await?;
            let status = page.get("status").and_then(|v| v.as_i64()).unwrap_or(0);
            let records: Vec<ReverseDomainRecord> = page
                .get("records")
                .and_then(|v| serde_json::from_value(v.clone()).ok())
                .unwrap_or_default();

            if status == 0 || status == 2 {
                remaining -= records.len() as i32;
                results.extend(records);
            }
            let exhausted = remaining <= 0;
            if status == 2 || exhausted {
                if exhausted {
                    let _ = self.terminate_search(search_id).await;
                }
                break;
            }
        }
        Ok(results)
    }
}
