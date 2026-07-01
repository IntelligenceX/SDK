//! CLI-level integration tests: spawn the built binary and assert on its behavior. These never
//! hit the network and require no credentials.

use assert_cmd::Command;
use predicates::str::contains;

fn cmd() -> Command {
    let mut cmd = Command::cargo_bin("intelx").unwrap();
    cmd.env_remove("INTELX_KEY");
    cmd.env_remove("INTELX_BASE_URL");
    cmd
}

#[test]
fn help_lists_all_subcommands() {
    cmd()
        .arg("--help")
        .assert()
        .success()
        .stdout(contains("search"))
        .stdout(contains("identity"))
        .stdout(contains("download"))
        .stdout(contains("capabilities"));
}

#[test]
fn search_without_api_key_fails_with_clear_message() {
    cmd()
        .args(["search", "test.com"])
        .assert()
        .failure()
        .stderr(contains("No API key specified"));
}

#[test]
fn missing_subcommand_fails_with_usage() {
    cmd().assert().failure().stderr(contains("Usage"));
}
