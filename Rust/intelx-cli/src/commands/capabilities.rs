use intelx::IntelXClient;

use crate::output;

pub async fn run(client: &IntelXClient) -> intelx::Result<()> {
    output::info("Getting your API capabilities.\n");
    let capabilities = client.get_capabilities().await?;
    output::print_json(&capabilities);
    Ok(())
}
