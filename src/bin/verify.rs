use std::{fs::File, path::PathBuf};

use anyhow::Result;
use clap::Parser;
use parquet_parser::reader::read_parquet;
use polars::{io::SerReader, prelude::ParquetReader};

#[derive(Parser)]
struct Cli {
    file: PathBuf,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let df = read_parquet(&cli.file)?;

    let df_from_polars = {
        let file = File::open(cli.file)?;
        let reader = ParquetReader::new(file);
        reader.finish()?
    };

    assert_eq!(df, df_from_polars);

    println!("Finish! Two dfs are the same");
    Ok(())
}
