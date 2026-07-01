use clap::{Args, Parser, Subcommand, ValueEnum};

/// Command-line client for https://intelx.io, built on the `intelx` crate.
#[derive(Debug, Parser)]
#[command(name = "intelx", version, about, long_about = None)]
pub struct Cli {
    /// API key. Falls back to the `INTELX_KEY` environment variable.
    #[arg(long, global = true)]
    pub api_key: Option<String>,

    /// Base URL override. Falls back to `INTELX_BASE_URL`, then the per-command default.
    #[arg(long, global = true)]
    pub base_url: Option<String>,

    /// Print raw JSON instead of formatted output.
    #[arg(long, global = true)]
    pub raw: bool,

    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub enum Command {
    /// Search the general Intelligence X index (intelligent search or phonebook search).
    Search(SearchArgs),
    /// Search the Identity Service for leaked accounts / reverse-domain activity.
    Identity(IdentityArgs),
    /// Download a single item by its system ID.
    Download(DownloadArgs),
    /// Show your account's API capabilities.
    Capabilities,
}

#[derive(Debug, Args)]
pub struct SearchArgs {
    /// The search term (a "strong selector": email, domain, IP, hash, etc).
    pub term: String,

    /// Comma-separated list of buckets to search.
    #[arg(long)]
    pub buckets: Option<String>,

    /// Maximum number of results to return. Defaults to 10 for display, 100 internally.
    #[arg(long)]
    pub limit: Option<i32>,

    /// Search timeout in seconds.
    #[arg(long, default_value_t = 5)]
    pub timeout: i32,

    /// Starting date filter, `YYYY-mm-dd HH:ii:ss`.
    #[arg(long)]
    pub datefrom: Option<String>,

    /// Ending date filter, `YYYY-mm-dd HH:ii:ss`.
    #[arg(long)]
    pub dateto: Option<String>,

    /// Media type filter (0 = all).
    #[arg(long, default_value_t = 0)]
    pub media: i32,

    /// Run a phonebook search instead, restricted to this selector type.
    #[arg(long, value_enum)]
    pub phonebook: Option<PhonebookKind>,

    /// With `--phonebook`, print only email selectors.
    #[arg(long)]
    pub emails: bool,

    /// Show the full contents of each result instead of a short preview.
    #[arg(long)]
    pub view: bool,

    /// Skip the text preview/view snippet entirely.
    #[arg(long)]
    pub nopreview: bool,

    /// Print bucket-count statistics instead of individual results.
    #[arg(long)]
    pub stats: bool,

    /// Export all matching files instead of printing results.
    #[arg(long)]
    pub export: bool,

    /// Export format, used with `--export`.
    #[arg(long, value_enum, default_value_t = ExportFormatArg::Zip)]
    pub export_format: ExportFormatArg,

    /// Directory to write exported/downloaded files to.
    #[arg(long, default_value = ".")]
    pub out_dir: std::path::PathBuf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum PhonebookKind {
    All,
    Domains,
    Emails,
    Urls,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum ExportFormatArg {
    Csv,
    Zip,
}

#[derive(Debug, Args)]
pub struct IdentityArgs {
    /// The domain or email address to look up.
    pub term: String,

    /// Maximum number of results to return.
    #[arg(long, default_value_t = 10)]
    pub limit: i32,

    /// Comma-separated list of buckets to search.
    #[arg(long)]
    pub buckets: Option<String>,

    /// Starting date filter, `YYYY-mm-dd HH:ii:ss`.
    #[arg(long)]
    pub datefrom: Option<String>,

    /// Ending date filter, `YYYY-mm-dd HH:ii:ss`.
    #[arg(long)]
    pub dateto: Option<String>,

    #[command(subcommand)]
    pub kind: IdentityKind,
}

#[derive(Debug, Subcommand)]
pub enum IdentityKind {
    /// Search for leaked data (`/live/search/internal`).
    DataLeaks,
    /// Export leaked accounts to a TSV file (`/accounts/csv`).
    ExportAccounts,
    /// Reverse-domain lookup (`/reverse/domain`).
    ReverseDomain,
}

#[derive(Debug, Args)]
pub struct DownloadArgs {
    /// System ID of the item to download.
    pub id: String,

    /// Bucket the item was found in.
    #[arg(long)]
    pub bucket: String,

    /// Filename to save the item as. Defaults to `<id>.bin`.
    #[arg(long)]
    pub name: Option<String>,
}
