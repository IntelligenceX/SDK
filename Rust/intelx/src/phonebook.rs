//! Phonebook search: `/phonebook/search*`.

use std::time::Duration;

use crate::client::IntelXClient;
use crate::error::{IntelXError, Result};
use crate::models::{
    IntelSearchStartResponse, PhonebookSearchParams, PhonebookSearchResultPage, PhonebookSelector,
    SearchStatus,
};

impl IntelXClient {
    /// Starts a phonebook search and returns its ID for further processing.
    ///
    /// Mirrors the Python SDK's `PHONEBOOK_SEARCH()`.
    pub async fn phonebook_search(&self, params: PhonebookSearchParams) -> Result<uuid::Uuid> {
        self.rate_limit_sleep().await;
        let request = params.into_request();
        let response: IntelSearchStartResponse =
            self.post_json("/phonebook/search", &request).await?;
        response.id.ok_or_else(|| IntelXError::Api {
            status: 200,
            message: "missing search id in response".into(),
        })
    }

    /// Fetches a page of results for a previously started phonebook search.
    ///
    /// Mirrors the Python SDK's `PHONEBOOK_SEARCH_RESULT()` / `query_pb_results()`.
    /// `offset` should normally be left at `-1` (the API default: each call returns the next
    /// available results); the Python SDK explicitly recommends not overriding it.
    pub async fn phonebook_search_result(
        &self,
        id: uuid::Uuid,
        limit: i32,
        offset: i32,
    ) -> Result<PhonebookSearchResultPage> {
        self.rate_limit_sleep().await;
        self.get(
            "/phonebook/search/result",
            &[
                ("id", id.to_string()),
                ("limit", limit.to_string()),
                ("offset", offset.to_string()),
            ],
        )
        .await
    }

    /// Runs a phonebook search to completion, polling until the API reports no more results
    /// (or `params.maxresults` has been satisfied), and returns every fetched page.
    ///
    /// Mirrors the Python SDK's high-level `phonebooksearch()`. Pages are kept separate (not
    /// flattened) to match Python's behavior of accumulating raw response pages; use
    /// [`flatten_selectors`] to get a single `Vec<PhonebookSelector>` if you don't need
    /// per-page status information.
    pub async fn phonebook_search_all(
        &self,
        params: PhonebookSearchParams,
    ) -> Result<Vec<PhonebookSearchResultPage>> {
        let mut remaining = params.maxresults;
        let search_id = self.phonebook_search(params).await?;
        let mut pages = Vec::new();

        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            let page = self
                .phonebook_search_result(search_id, remaining, -1)
                .await?;
            remaining -= page.selectors.len() as i32;
            let status = page.status;
            pages.push(page);

            let exhausted = remaining <= 0;
            if exhausted {
                let _ = self.intel_terminate_search(search_id).await;
            }
            if exhausted || matches!(status, SearchStatus::NoMoreResults | SearchStatus::NotFound) {
                break;
            }
        }

        Ok(pages)
    }
}

/// Flattens the pages returned by [`IntelXClient::phonebook_search_all`] into a single list of
/// selectors.
pub fn flatten_selectors(pages: &[PhonebookSearchResultPage]) -> Vec<PhonebookSelector> {
    pages
        .iter()
        .flat_map(|page| page.selectors.iter().cloned())
        .collect()
}
