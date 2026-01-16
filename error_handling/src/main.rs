use std::collections::BTreeMap;
use std::path::PathBuf;

use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    name = "csv-headers",
    about = "Read and print CSV headers from a file",
    version
)]
struct Args {
    #[arg(short, long)]
    path: PathBuf,
}

fn main() -> Result<()> {
    // unsafe { std::env::set_var("RUST_BACKTRACE", "1") };
    let args = Args::parse();

    let mut reader = csv::Reader::from_path(&args.path)
        .with_context(|| format!("Failed to open CSV file: {}", args.path.display()))?;

    let mut records = reader.deserialize::<BTreeMap<String, String>>();
    let first_row = records
        .next()
        .transpose()
        .context("Failed to deserialize the first CSV row")?
        .ok_or_else(|| anyhow::anyhow!("CSV file has no data rows to deserialize"))?;

    let headers = first_row.keys().cloned().collect::<Vec<_>>();
    println!("{}", headers.join(","));
    Ok(())
}
