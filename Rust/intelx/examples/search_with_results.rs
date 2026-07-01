#[tokio::main]
async fn main() -> intelx::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");
    let mut builder = intelx::IntelXClient::builder().api_key(api_key);
    if let Ok(base_url) = std::env::var("INTELX_BASE_URL") {
        builder = builder.base_url(base_url);
    }
    let client = builder.build()?;

    let params = intelx::SearchParams::new("riseup.net")
        .maxresults(50)
        .buckets(["leaks.public", "pastes"])
        .timeout(5)
        .datefrom("2021-01-01 00:00:00")
        .dateto("2022-02-02 23:00:00")
        .sort(intelx::SortOrder::DateDesc)
        .media(0);

    let search_id = client.intel_search(params).await?;
    println!("Search ID: {search_id}");

    let limit = 100;
    loop {
        let page = client.intel_search_result(search_id, limit).await?;
        println!("Status: {:?}, records: {}", page.status, page.records.len());

        for record in &page.records {
            println!("{} {}", record.name, record.bucket);
        }

        if matches!(
            page.status,
            intelx::SearchStatus::NoMoreResults | intelx::SearchStatus::NotFound
        ) {
            break;
        }
    }

    Ok(())
}
