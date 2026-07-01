# intelx Rust SDK

## Introduction

`intelx` is an async Rust SDK and command-line client for [intelx.io](https://intelx.io),
the Intelligence X search engine and data archive. It is a Rust port of the
[Python `intelx` SDK](../Python), covering intelligent search, phonebook search, file
operations, and the Identity Service (leaked-account search and reverse-domain lookup).

The workspace has two crates:

* [`intelx/`](intelx) - the library crate.
* [`intelx-cli/`](intelx-cli) - a `clap`-based command-line client built on the library,
  producing the `intelx` binary.

## Requirements

* Rust **1.96.0** (pinned via [`rust-toolchain.toml`](rust-toolchain.toml); `rustup` will
  install it automatically).

## Installation

### As a library

From within this monorepo (before the crate is published to crates.io), add a path
dependency:

```toml
[dependencies]
intelx = { path = "../Rust/intelx" }
```

Once published, use the registry dependency instead:

```bash
cargo add intelx
```

### The CLI

```bash
cargo install --path Rust/intelx-cli
```

This installs a binary named `intelx`.

## Setup

You will need an API key from <https://intelx.io/account?tab=developer>.

### Environment variable

Copy `.env.sample` to `.env` and set your values:

```bash
INTELX_KEY="00000000-0000-0000-0000-000000000000"
INTELX_BASE_URL="https://2.intelx.io"
```

Examples and the CLI both load `.env` automatically via the [`dotenvy`](https://docs.rs/dotenvy)
crate.

### Via the CLI

```bash
export INTELX_KEY=00000000-0000-0000-0000-000000000000
intelx search riseup.net
```

or pass it explicitly:

```bash
intelx --api-key "$INTELX_KEY" search riseup.net
```

## Usage as a CLI

```bash
# Quick search
intelx search riseup.net

# Search in specific buckets
intelx search riseup.net --buckets "pastes,darknet.tor"

# Search with 100 results
intelx search riseup.net --limit 100

# Download an item (requires --bucket)
intelx download 29a97791-1138-40b3-8cf1-de1764e9d09c \
  --bucket leaks.private.general --name test.txt

# View the full contents of a search result instead of a short preview
intelx search 3a4d5699-737c-4d22-8dbd-c5391ce805df --view

# Export all matching files from a search
intelx search email@email.com --export --export-format zip --limit 5 \
  --buckets "pastes,leaks.private.general,leaks.logs,whois,usenet"

# Extract emails from a phonebook search
intelx search cia.gov --phonebook emails

# Identity Portal: export leaked accounts
intelx identity riseup.net export-accounts

# Identity Portal: data leaks search
intelx identity riseup.net data-leaks

# Account capabilities
intelx capabilities
```

Pass `--raw` to any command to print JSON instead of formatted output.

## Usage as a library

```rust
let client = intelx::IntelXClient::new("00000000-0000-0000-0000-000000000000")?;
let results = client.search(intelx::SearchParams::new("hackerone.com")).await?;
```

### Advanced search

By default `maxresults` is `100`. The following parameters all have defaults but can be
overridden via the [`SearchParams`](intelx/src/models/search.rs) builder:

* `maxresults` = 100
* `buckets` = `[]`
* `timeout` = 5 (seconds)
* `datefrom` / `dateto` = `""`
* `sort` = `SortOrder::DateDesc`
* `media` = 0
* `terminate` = `[]`

```rust
let results = client
    .search(intelx::SearchParams::new("hackerone.com").maxresults(200))
    .await?;
```

#### Searching in specific buckets

```rust
let params = intelx::SearchParams::new("hackerone.com")
    .maxresults(200)
    .buckets(["darknet", "leaks.public", "leaks.private"]);
let results = client.search(params).await?;
```

Your account must have access to every specified bucket, otherwise you will receive
`401 Unauthorized`. The `leaks.private` bucket is only available on certain licenses.

#### Filtering by date

```rust
let params = intelx::SearchParams::new("riseup.net")
    .maxresults(200)
    .datefrom("2014-01-01 00:00:00")
    .dateto("2014-02-02 23:59:59");
let results = client.search(params).await?;
```

#### Filtering by media type

See the [Media Types](#media-types) table below for the available IDs.

```rust
let params = intelx::SearchParams::new("riseup.net")
    .maxresults(200)
    .media(1) // Paste document
    .datefrom("2014-01-01 00:00:00")
    .dateto("2014-02-02 23:59:59");
let results = client.search(params).await?;
```

#### Statistics

```rust
let results = client
    .search(intelx::SearchParams::new("riseup.net").maxresults(1000))
    .await?;
let stats = intelx::stats(&results);
println!("{stats:?}");
```

### Viewing/reading files

There is a fundamental difference between `file_view` and `file_read`: viewing is for
quickly inspecting the contents of a file (assumed to be text); `file_read` is for direct
data download, reliably returning binary contents (ZIP, PDF, etc) without encoding issues.

#### Viewing

```rust
let results = client.search(intelx::SearchParams::new("riseup.net")).await?;
let first = &results[0];
let contents = client
    .file_view(first.item_type, first.media, &first.storageid, &first.bucket, 0)
    .await?;
println!("{contents}");
```

#### Reading

```rust
let results = client.search(intelx::SearchParams::new("riseup.net")).await?;
let first = &results[0];
client
    .file_read(&first.systemid.to_string(), intelx::FileReadType::Raw, &first.bucket,
               std::path::Path::new("file.bin"))
    .await?;
```

### Date handling

Date fields (`Item::added`, `Item::date`) are kept as plain `String`s rather than
`chrono::DateTime`, because the server's `YYYY-mm-dd HH:ii:ss` format is not RFC 3339.
Parse them yourself with `chrono::NaiveDateTime::parse_from_str(&item.date, "%Y-%m-%d %H:%M:%S")`
if you need typed dates.

## Other notes

### Media Types

| ID | Media Type                        |
|----|------------------------------------|
| 0  | All                                |
| 1  | Paste document                     |
| 2  | Paste user                         |
| 3  | Forum                              |
| 4  | Forum board                        |
| 5  | Forum thread                       |
| 6  | Forum post                         |
| 7  | Forum user                         |
| 8  | Screenshot of website               |
| 9  | HTML copy of website               |
| 13 | Tweet                               |
| 14 | URL                                 |
| 15 | PDF document                        |
| 16 | Word document                       |
| 17 | Excel document                      |
| 18 | Powerpoint document                 |
| 19 | Picture                             |
| 20 | Audio file                          |
| 21 | Video file                          |
| 22 | Container file (ZIP/RAR/TAR, etc)   |
| 23 | HTML file                           |
| 24 | Text file                           |

### Format Types

| ID | Format Type                         |
|----|---------------------------------------|
| 0  | textview of content                   |
| 1  | hex view of content                   |
| 2  | auto detect hex view or text view     |
| 3  | picture view                          |
| 4  | not supported                         |
| 5  | html inline view (sanitized)          |
| 6  | text view of pdf                      |
| 7  | text view of html                     |
| 8  | text view of word file                |

## Testing

```bash
cd Rust
cargo test --workspace            # unit + wiremock + CLI tests, no network/credentials
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all -- --check
INTELX_KEY=... cargo test -p intelx --test live_smoke -- --ignored  # live API smoke test
```

# Contribute

Please use the [issue tracker](https://github.com/IntelligenceX/SDK/issues) to report any
bugs, security vulnerabilities, or feature requests.