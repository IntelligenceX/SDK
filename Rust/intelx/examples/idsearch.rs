#[tokio::main]
async fn main() -> intelx::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");

    let identity = intelx::IdentityClient::new(api_key.clone())?;
    let general = intelx::IntelXClient::new(api_key)?;

    let records = identity
        .idsearch(intelx::IdSearchParams::new("john.doe@example.com"))
        .await?;
    let first_item = records
        .first()
        .and_then(|record| record.item.as_ref())
        .expect("identity search returned no usable results");

    let contents = general
        .file_view(
            first_item.item_type,
            first_item.media,
            &first_item.storageid,
            &first_item.bucket,
            0,
        )
        .await?;
    println!("{contents}");

    Ok(())
}
