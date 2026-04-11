use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{file_metadata::read_file_metadata, row_group::read_row_groups};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn one_run_div_by_8() -> Result<()> {
    let data = make_parquet(
        r#"
boolean
true
true
true
true
true
true
true
true
"#,
        false,
        Encoding::RLE,
        Compression::UNCOMPRESSED,
        None,
        None,
        None,
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "boolean" => [true, true, true, true, true, true, true, true],
    )?;
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn one_run_not_div_by_8() -> Result<()> {
    let data = make_parquet(
        r#"
boolean
true
true
true
true
true
true
"#,
        false,
        Encoding::RLE,
        Compression::UNCOMPRESSED,
        None,
        None,
        None,
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "boolean" => [true, true, true, true, true, true],
    )?;
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn many_runs() -> Result<()> {
    let run_len = 5000;
    let boolean_data: Vec<bool> = (0..run_len).map(|i| i < run_len / 2).collect();

    let data: Vec<&str> = std::iter::once("boolean") // header
        .chain(
            boolean_data
                .iter()
                .map(|value| if *value { "true" } else { "false" }),
        )
        .collect();
    let data = make_parquet(
        &data.join("\n"),
        false,
        Encoding::RLE,
        Compression::UNCOMPRESSED,
        None,
        None,
        None,
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "boolean" => boolean_data,
    )?;
    assert_eq!(df, expected);

    Ok(())
}
