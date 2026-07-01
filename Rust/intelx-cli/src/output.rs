use colored::Colorize;
use comfy_table::Table;

pub const BANNER: &str = r#"
     _____      _       ___   __
    |_   _|    | |     | \ \ / /
      | | _ __ | |_ ___| |\ V /
      | || '_ \| __/ _ \ |/   \
     _| || | | | ||  __/ / /^\ \
     \___/_| |_|\__\___|_\/   \/

       a command line client
           for intelx.io
"#;

fn timestamp() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs_today = now.as_secs() % 86400;
    format!(
        "{:02}:{:02}:{:02}",
        secs_today / 3600,
        (secs_today % 3600) / 60,
        secs_today % 60
    )
}

pub fn info(message: &str) {
    println!("{}", format!("[{}] {message}", timestamp()).green());
}

pub fn warn(message: &str) {
    println!("{}", format!("[{}] {message}", timestamp()).yellow());
}

pub fn error(message: &str) {
    eprintln!("{}", format!("[{}] {message}", timestamp()).red());
}

pub fn table(headers: Vec<&str>, rows: Vec<Vec<String>>) {
    let mut table = Table::new();
    table.set_header(headers);
    for row in rows {
        table.add_row(row);
    }
    println!("{table}");
}

pub fn print_json<T: serde::Serialize>(value: &T) {
    match serde_json::to_string_pretty(value) {
        Ok(json) => println!("{json}"),
        Err(err) => error(&format!("failed to serialize output as JSON: {err}")),
    }
}
