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
        .search(intelx::SearchParams::new("riseup.net").maxresults(1000))
        .await?;
    let stats = intelx::stats(&results);
    println!("{stats:#?}");

    Ok(())
}
