fn client_from_env() -> intelx::Result<intelx::IntelXClient> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");
    let mut builder = intelx::IntelXClient::builder().api_key(api_key);
    if let Ok(base_url) = std::env::var("INTELX_BASE_URL") {
        builder = builder.base_url(base_url);
    }
    builder.build()
}

async fn count_in_buckets(
    client: &intelx::IntelXClient,
    target: &str,
    buckets: &[&str],
    label: &str,
) -> intelx::Result<()> {
    let params = intelx::SearchParams::new(target)
        .buckets(buckets.iter().copied())
        .maxresults(2000);
    let results = client.search(params).await?;
    println!(
        "Found {} records for {target} in bucket '{label}'",
        results.len()
    );
    Ok(())
}

#[tokio::main]
async fn main() -> intelx::Result<()> {
    let client = client_from_env()?;
    let target = "riseup.net";

    count_in_buckets(&client, target, &["leaks.public", "leaks.private"], "leaks").await?;
    count_in_buckets(&client, target, &["pastes"], "pastes").await?;
    count_in_buckets(&client, target, &["darknet"], "darknet").await?;

    Ok(())
}
