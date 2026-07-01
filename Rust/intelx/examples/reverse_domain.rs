#[tokio::main]
async fn main() -> intelx::Result<()> {
    dotenvy::dotenv().ok();
    let api_key = std::env::var("INTELX_KEY").expect("INTELX_KEY must be set");
    let identity = intelx::IdentityClient::new(api_key)?;

    let params = intelx::ReverseDomainParams::new("riseup.net")
        .maxresults(10)
        .datefrom("2022-01-01 00:00:00")
        .dateto("2022-06-01 00:00:00");
    let results = identity.reverse_domain(params).await?;

    println!("{results:#?}");

    Ok(())
}
