//! Smoke tests against the real Intelligence X API.
//!
//! These are `#[ignore]`d by default and additionally bail out unless `INTELX_KEY` is set, so
//! `cargo test` never touches the network. Run explicitly with:
//!
//! ```text
//! INTELX_KEY=... cargo test -p intelx --test live_smoke -- --ignored
//! ```

use intelx::{IntelXClient, SearchParams};

fn live_client() -> Option<IntelXClient> {
    let api_key = std::env::var("INTELX_KEY").ok()?;
    let base_url = std::env::var("INTELX_BASE_URL").ok();
    let mut builder = IntelXClient::builder().api_key(api_key);
    if let Some(base_url) = base_url {
        builder = builder.base_url(base_url);
    }
    builder.build().ok()
}

#[tokio::test]
#[ignore]
async fn search_against_live_api() {
    let Some(client) = live_client() else {
        eprintln!("skipping: INTELX_KEY not set");
        return;
    };

    let results = client
        .search(SearchParams::new("riseup.net").maxresults(5))
        .await
        .expect("live search should succeed with a valid key");

    assert!(!results.is_empty());
}
