mod cli;
mod commands;
mod output;

use clap::Parser;
use cli::{Cli, Command};
use colored::Colorize;

fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

#[tokio::main]
async fn main() -> std::process::ExitCode {
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    if !cli.raw {
        println!("{}", output::BANNER.bold());
        println!("intelx v{}", version());
    }

    let api_key = cli
        .api_key
        .clone()
        .or_else(|| std::env::var("INTELX_KEY").ok());
    let Some(api_key) = api_key else {
        output::error(
            "No API key specified. Please use the \"--api-key\" parameter or set the environment variable \"INTELX_KEY\".",
        );
        return std::process::ExitCode::FAILURE;
    };

    let base_url = cli
        .base_url
        .clone()
        .or_else(|| std::env::var("INTELX_BASE_URL").ok());

    let result = run(api_key, base_url, cli).await;
    match result {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(err) => {
            output::error(&err.to_string());
            std::process::ExitCode::FAILURE
        }
    }
}

async fn run(api_key: String, base_url: Option<String>, cli: Cli) -> intelx::Result<()> {
    match cli.command {
        Command::Search(args) => {
            let mut builder = intelx::IntelXClient::builder().api_key(api_key);
            if let Some(base_url) = base_url {
                builder = builder.base_url(base_url);
            }
            let client = builder.build()?;
            commands::search::run(&client, args, cli.raw).await
        }
        Command::Identity(args) => {
            let mut builder = intelx::IdentityClient::builder().api_key(api_key);
            if let Some(base_url) = base_url {
                builder = builder.base_url(base_url);
            }
            let client = builder.build()?;
            commands::identity::run(&client, args, cli.raw).await
        }
        Command::Download(args) => {
            let mut builder = intelx::IntelXClient::builder().api_key(api_key);
            if let Some(base_url) = base_url {
                builder = builder.base_url(base_url);
            }
            let client = builder.build()?;
            commands::download::run(&client, args).await
        }
        Command::Capabilities => {
            let mut builder = intelx::IntelXClient::builder().api_key(api_key);
            if let Some(base_url) = base_url {
                builder = builder.base_url(base_url);
            }
            let client = builder.build()?;
            commands::capabilities::run(&client).await
        }
    }
}
