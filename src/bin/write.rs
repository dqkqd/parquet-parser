use std::collections::HashMap;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use clap::Parser;
use parquet::basic::{Compression, Encoding};
use parquet_parser::format::Type;
use parquet_parser::writer::write_parquet;

fn main() -> Result<()> {
    let cli = Cli::parse();
    make_parquet(cli)?;
    Ok(())
}

#[derive(Parser, Debug)]
struct Cli {
    input: PathBuf,

    output: PathBuf,

    #[arg(long, default_value = "Hello parquet!")]
    author: String,

    #[arg(long, default_value_t = false)]
    dictionary: bool,

    #[arg(long, default_value = "plain", value_parser = parse_encoding)]
    encoding: Encoding,

    #[arg(long, default_value = "uncompressed", value_parser = parse_compression)]
    compression: Compression,

    #[arg(long)]
    rows_per_page: Option<usize>,

    #[arg(long)]
    rows_per_group: Option<usize>,

    #[arg(long, value_parser = parse_column_dtype)]
    dtypes: Vec<(String, Type)>,
}

fn parse_encoding(s: &str) -> Result<Encoding> {
    match s.trim() {
        "plain" => Ok(Encoding::PLAIN),
        e => bail!("Unsupported encoding {e}, expected one of [plain]"),
    }
}

fn parse_compression(s: &str) -> Result<Compression> {
    match s.trim() {
        "uncompressed" => Ok(Compression::UNCOMPRESSED),
        e => bail!("Unsupported compression {e}, expected one of [uncompressed]"),
    }
}

fn parse_column_dtype(s: &str) -> Result<(String, Type)> {
    let (column, data_type) = s
        .split_once('=')
        .with_context(|| "Invalid column=data_type: no `=` found in {s}")?;
    let data_type = match data_type {
        "boolean" => Type::BOOLEAN,
        "int32" => Type::INT32,
        "int64" => Type::INT64,
        "float" => Type::FLOAT,
        "double" => Type::DOUBLE,
        "string" => Type::BYTE_ARRAY,
        _ => {
            bail!("Unsupported data type, expected: [boolean, int32, int64, float, double, string]")
        }
    };
    Ok((column.to_string(), data_type))
}

fn make_parquet(cli: Cli) -> Result<()> {
    let mut file = File::open(cli.input)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;

    let data_types_override: HashMap<String, Type> = cli.dtypes.clone().into_iter().collect();

    let out = write_parquet(
        data,
        cli.dictionary,
        cli.encoding,
        cli.compression,
        cli.rows_per_page,
        cli.rows_per_group,
        Some(data_types_override),
    )?;

    let mut file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(cli.output)?;
    file.write_all(&out)?;
    Ok(())
}
