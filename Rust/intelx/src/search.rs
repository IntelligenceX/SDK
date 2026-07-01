//! Intelligent search: `/intelligent/search*`.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

use crate::client::IntelXClient;
use crate::error::{IntelXError, Result};
use crate::models::{
    ExportFormat, IntelSearchResultPage, IntelSearchStartResponse, SearchParams, SearchResult,
    SearchStatus,
};
use crate::util::parse_content_disposition_filename;

impl IntelXClient {
    /// Starts an intelligent search and returns its ID for further processing.
    ///
    /// Mirrors the Python SDK's `INTEL_SEARCH()`. Returns [`IntelXError::InvalidTerm`] if the
    /// API rejects `params.term` as not being a supported "strong selector", or
    /// [`IntelXError::MaxConcurrentSearches`] if the account has too many active searches.
    pub async fn intel_search(&self, params: SearchParams) -> Result<uuid::Uuid> {
        self.rate_limit_sleep().await;
        let request = params.into_request();
        let response: IntelSearchStartResponse =
            self.post_json("/intelligent/search", &request).await?;
        match response.status {
            1 => Err(IntelXError::InvalidTerm),
            2 => Err(IntelXError::MaxConcurrentSearches),
            _ => response.id.ok_or_else(|| IntelXError::Api {
                status: 200,
                message: "missing search id in response".into(),
            }),
        }
    }

    /// Fetches a page of results for a previously started intelligent search.
    ///
    /// Mirrors the Python SDK's `INTEL_SEARCH_RESULT()` / `query_results()`.
    pub async fn intel_search_result(
        &self,
        id: uuid::Uuid,
        limit: i32,
    ) -> Result<IntelSearchResultPage> {
        self.rate_limit_sleep().await;
        self.get(
            "/intelligent/search/result",
            &[("id", id.to_string()), ("limit", limit.to_string())],
        )
        .await
    }

    /// Terminates a previously started intelligent search.
    ///
    /// Mirrors the Python SDK's `INTEL_TERMINATE_SEARCH()`.
    pub async fn intel_terminate_search(&self, id: uuid::Uuid) -> Result<()> {
        self.rate_limit_sleep().await;
        let response = self
            .get_response("/intelligent/search/terminate", &[("id", id.to_string())])
            .await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(crate::error::api_error_from_status(response.status()))
        }
    }

    /// Exports all files from a search to `dest_dir`, mirroring the Python SDK's
    /// `INTEL_EXPORT()`. The filename is taken from the response's `Content-Disposition`
    /// header; returns the resolved path it was written to.
    pub async fn intel_export(
        &self,
        id: uuid::Uuid,
        format: ExportFormat,
        limit: i32,
        dest_dir: &Path,
    ) -> Result<PathBuf> {
        self.rate_limit_sleep().await;
        let format_code = format as i32;
        let response = self
            .get_response(
                "/intelligent/search/export",
                &[
                    ("id", id.to_string()),
                    ("f", format_code.to_string()),
                    ("l", limit.to_string()),
                ],
            )
            .await?;

        if !response.status().is_success() {
            return Err(crate::error::api_error_from_status(response.status()));
        }

        let filename = response
            .headers()
            .get(reqwest::header::CONTENT_DISPOSITION)
            .and_then(|v| v.to_str().ok())
            .and_then(parse_content_disposition_filename)
            .ok_or(IntelXError::MissingFilename)?;

        let dest_path = dest_dir.join(filename);
        let bytes = response.bytes().await?;
        tokio::fs::write(&dest_path, &bytes).await?;
        Ok(dest_path)
    }

    /// Lists all selectors found within a document.
    ///
    /// Mirrors the Python SDK's `selectors()`.
    pub async fn selectors(&self, document: uuid::Uuid) -> Result<Vec<serde_json::Value>> {
        self.rate_limit_sleep().await;
        let value: serde_json::Value = self
            .get("/item/selector/list/human", &[("id", document.to_string())])
            .await?;
        Ok(value
            .get("selectors")
            .and_then(|v| v.as_array())
            .cloned()
            .unwrap_or_default())
    }

    /// Runs an intelligent search to completion, polling until the API reports no more results
    /// (or `params.maxresults` has been satisfied), and returns the accumulated records.
    ///
    /// Mirrors the Python SDK's high-level `search()`. The search is terminated server-side if
    /// it's abandoned early because `maxresults` was reached, matching Python's behavior.
    ///
    /// This is the recommended entry point for most callers; for custom polling cadence or
    /// early-exit logic, compose [`IntelXClient::intel_search`],
    /// [`IntelXClient::intel_search_result`], and [`IntelXClient::intel_terminate_search`]
    /// directly.
    pub async fn search(&self, params: SearchParams) -> Result<Vec<SearchResult>> {
        let mut remaining = params.maxresults;
        let search_id = self.intel_search(params).await?;
        let mut results = Vec::new();

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let page = self.intel_search_result(search_id, remaining).await?;
            remaining -= page.records.len() as i32;
            results.extend(page.records);

            let exhausted = remaining <= 0;
            if exhausted {
                let _ = self.intel_terminate_search(search_id).await;
            }
            if exhausted
                || matches!(
                    page.status,
                    SearchStatus::NoMoreResults | SearchStatus::NotFound
                )
            {
                break;
            }
        }

        Ok(results)
    }

    /// Starts a search and exports its results to `dest_dir`. Mirrors the Python SDK's
    /// `exportfromsearch()`.
    pub async fn export_from_search(
        &self,
        params: SearchParams,
        format: ExportFormat,
        dest_dir: &Path,
    ) -> Result<PathBuf> {
        let limit = params.maxresults;
        let search_id = self.intel_search(params).await?;
        self.intel_export(search_id, format, limit, dest_dir).await
    }
}

/// Counts results by bucket. Mirrors the Python SDK's `stats()`.
///
/// This is a pure function operating on already-fetched results, so it does not require a
/// client or perform any I/O.
pub fn stats(results: &[SearchResult]) -> BTreeMap<String, usize> {
    let mut counts = BTreeMap::new();
    for result in results {
        *counts.entry(result.bucket.clone()).or_insert(0) += 1;
    }
    counts
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::Item;

    fn result_with_bucket(bucket: &str) -> SearchResult {
        SearchResult {
            item: Item {
                systemid: uuid::Uuid::new_v4(),
                storageid: String::new(),
                instore: false,
                size: 0,
                accesslevel: 0,
                item_type: 0,
                media: 0,
                added: String::new(),
                date: String::new(),
                name: String::new(),
                description: String::new(),
                xscore: 0,
                simhash: 0,
                bucket: bucket.to_string(),
                tags: Vec::new(),
                relations: Vec::new(),
            },
            accesslevelh: String::new(),
            mediah: String::new(),
            simhashh: String::new(),
            typeh: String::new(),
            tagsh: Vec::new(),
            randomid: None,
            bucketh: String::new(),
            group: String::new(),
            indexfile: String::new(),
        }
    }

    #[test]
    fn stats_counts_records_by_bucket() {
        let results = vec![
            result_with_bucket("pastes"),
            result_with_bucket("pastes"),
            result_with_bucket("darknet.i2p"),
        ];
        let counts = stats(&results);
        assert_eq!(counts.get("pastes"), Some(&2));
        assert_eq!(counts.get("darknet.i2p"), Some(&1));
    }
}
