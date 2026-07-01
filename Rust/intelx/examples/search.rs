fn client_from_env() -> intelx::Result<intelx::IntelXClient> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");
    let mut builder = intelx::IntelXClient::builder().api_key(api_key);
    if let Ok(base_url) = std::env::var("INTELX_BASE_URL") {
        builder = builder.base_url(base_url);
    }
    builder.build()
}

#[tokio::main]
async fn main() -> intelx::Result<()> {
    let client = client_from_env()?;

    let results = client
        .search(intelx::SearchParams::new("riseup.net"))
        .await?;
    for record in &results {
        println!("Found media type {} in {}", record.media, record.bucket);
    }

    Ok(())
}
