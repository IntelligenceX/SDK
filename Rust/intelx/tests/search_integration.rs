//! HTTP-layer integration tests using `wiremock`. These never hit the live API and require no
//! credentials, so they run in the default `cargo test`.

use intelx::{IntelXClient, IntelXError, SearchParams};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

async fn client_for(mock_server: &MockServer) -> IntelXClient {
    IntelXClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .rate_limit(std::time::Duration::from_millis(0))
        .build()
        .unwrap()
}

#[tokio::test]
async fn intel_search_returns_parsed_search_id() {
    let mock_server = MockServer::start().await;
    let search_id = "61202067-543e-4e6a-8c23-11f9b8f008cf";

    Mock::given(method("POST"))
        .and(path("/intelligent/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": search_id,
            "status": 0,
            "softselectorwarning": false
        })))
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let id = client
        .intel_search(SearchParams::new("riseup.net"))
        .await
        .unwrap();
    assert_eq!(id.to_string(), search_id);
}

#[tokio::test]
async fn intel_search_maps_invalid_term_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/intelligent/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": null,
            "status": 1
        })))
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let err = client
        .intel_search(SearchParams::new("not a selector"))
        .await
        .unwrap_err();
    assert!(matches!(err, IntelXError::InvalidTerm));
}

#[tokio::test]
async fn intel_search_maps_unauthorized_response() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/intelligent/search"))
        .respond_with(ResponseTemplate::new(401))
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let err = client
        .intel_search(SearchParams::new("riseup.net"))
        .await
        .unwrap_err();
    match err {
        IntelXError::Api { status, .. } => assert_eq!(status, 401),
        other => panic!("expected Api error, got {other:?}"),
    }
}

#[tokio::test]
async fn search_polls_across_multiple_pages_and_accumulates_records() {
    let mock_server = MockServer::start().await;
    let search_id = "61202067-543e-4e6a-8c23-11f9b8f008cf";

    Mock::given(method("POST"))
        .and(path("/intelligent/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": search_id,
            "status": 0,
            "softselectorwarning": false
        })))
        .mount(&mock_server)
        .await;

    // First page: status 0 (more results available), one record.
    Mock::given(method("GET"))
        .and(path("/intelligent/search/result"))
        .and(query_param("id", search_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": 0,
            "records": [{
                "systemid": "11111111-1111-1111-1111-111111111111",
                "bucket": "pastes"
            }]
        })))
        .up_to_n_times(1)
        .mount(&mock_server)
        .await;

    // Second page: status 1 (no more results), one more record.
    Mock::given(method("GET"))
        .and(path("/intelligent/search/result"))
        .and(query_param("id", search_id))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": 1,
            "records": [{
                "systemid": "22222222-2222-2222-2222-222222222222",
                "bucket": "darknet.i2p"
            }]
        })))
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let results = client
        .search(SearchParams::new("riseup.net").maxresults(100))
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].bucket, "pastes");
    assert_eq!(results[1].bucket, "darknet.i2p");
}

#[tokio::test]
async fn search_terminates_search_when_maxresults_exhausted_early() {
    let mock_server = MockServer::start().await;
    let search_id = "61202067-543e-4e6a-8c23-11f9b8f008cf";

    Mock::given(method("POST"))
        .and(path("/intelligent/search"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "id": search_id,
            "status": 0
        })))
        .mount(&mock_server)
        .await;

    // Single page already satisfies maxresults=1, while status still says "more available".
    Mock::given(method("GET"))
        .and(path("/intelligent/search/result"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "status": 0,
            "records": [{
                "systemid": "11111111-1111-1111-1111-111111111111",
                "bucket": "pastes"
            }]
        })))
        .mount(&mock_server)
        .await;

    Mock::given(method("GET"))
        .and(path("/intelligent/search/terminate"))
        .and(query_param("id", search_id))
        .respond_with(ResponseTemplate::new(200))
        .expect(1)
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let results = client
        .search(SearchParams::new("riseup.net").maxresults(1))
        .await
        .unwrap();
    assert_eq!(results.len(), 1);

    // wiremock verifies the `expect(1)` terminate call was made when the MockServer is dropped.
}

#[tokio::test]
async fn file_read_streams_response_body_to_disk() {
    let mock_server = MockServer::start().await;
    let payload = b"hello from intelx";

    Mock::given(method("GET"))
        .and(path("/file/read"))
        .respond_with(ResponseTemplate::new(200).set_body_bytes(payload.to_vec()))
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let dir = tempfile::tempdir().unwrap();
    let dest = dir.path().join("downloaded.bin");

    let written = client
        .file_read("system-id", intelx::FileReadType::Raw, "pastes", &dest)
        .await
        .unwrap();

    assert_eq!(written, payload.len() as u64);
    let contents = std::fs::read(&dest).unwrap();
    assert_eq!(contents, payload);
}

#[tokio::test]
async fn intel_export_writes_file_named_from_content_disposition() {
    let mock_server = MockServer::start().await;
    let search_id = "61202067-543e-4e6a-8c23-11f9b8f008cf";
    let payload = b"id,name\n1,test\n";

    Mock::given(method("GET"))
        .and(path("/intelligent/search/export"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header(
                    "Content-Disposition",
                    "attachment; filename=\"Search 2024.csv\"",
                )
                .set_body_bytes(payload.to_vec()),
        )
        .mount(&mock_server)
        .await;

    let client = client_for(&mock_server).await;
    let dir = tempfile::tempdir().unwrap();

    let path = client
        .intel_export(
            search_id.parse().unwrap(),
            intelx::ExportFormat::Csv,
            100,
            dir.path(),
        )
        .await
        .unwrap();

    assert_eq!(path.file_name().unwrap(), "Search 2024.csv");
    assert_eq!(std::fs::read(&path).unwrap(), payload);
}
