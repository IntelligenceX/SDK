use intelx::IntelXClient;

use crate::cli::DownloadArgs;
use crate::output;

pub async fn run(client: &IntelXClient, args: DownloadArgs) -> intelx::Result<()> {
    let filename = args.name.unwrap_or_else(|| format!("{}.bin", args.id));
    let dest = std::path::Path::new(&filename);

    match client
        .file_read(&args.id, intelx::FileReadType::Raw, &args.bucket, dest)
        .await
    {
        Ok(_) => {
            output::info(&format!("Successfully downloaded the file '{filename}'."));
            Ok(())
        }
        Err(err) => {
            output::error(&format!("Failed to download item {}: {err}", args.id));
            Err(err)
        }
    }
}
