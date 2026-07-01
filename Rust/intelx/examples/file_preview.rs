#[tokio::main]
async fn main() -> intelx::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");
    let mut builder = intelx::IntelXClient::builder().api_key(api_key);
    if let Ok(base_url) = std::env::var("INTELX_BASE_URL") {
        builder = builder.base_url(base_url);
    }
    let client = builder.build()?;

    let results = client
        .search(intelx::SearchParams::new("riseup.net").maxresults(5))
        .await?;
    let first = results.first().expect("search returned no results");

    let params = intelx::FilePreviewParams::new(1, first.media, 0, first.storageid.clone())
        .bucket(first.bucket.clone())
        .lines(20);
    let preview = client.file_preview(params).await?;
    println!("{preview}");

    Ok(())
}
