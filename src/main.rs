use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;
use parquet_parser::reader::read_parquet;

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let df = read_parquet(cli.file)?;
    println!("{df}");
    Ok(())
}
