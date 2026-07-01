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
        .search(intelx::SearchParams::new("riseup.net"))
        .await?;
    let first = results.first().expect("search returned no results");

    let dest = std::path::Path::new("file1.bin");
    client
        .file_read(
            &first.systemid.to_string(),
            intelx::FileReadType::Raw,
            &first.bucket,
            dest,
        )
        .await?;
    println!("Saved first search result to {}", dest.display());

    Ok(())
}
