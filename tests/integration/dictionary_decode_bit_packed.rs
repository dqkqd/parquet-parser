use std::collections::HashMap;

use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{file_metadata::read_file_metadata, format::Type, row_group::read_row_groups};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn one_run_one_page() -> Result<()> {
    let data = make_parquet(
        r#"
i32
10
20
30
40
50
60
"#,
        true,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i32", Type::INT32)])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "i32" => [10i32, 20i32, 30i32, 40i32, 50i32, 60i32],
    )?;
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn many_runs() -> Result<()> {
    let run_len = 3000;
    let i32_data: Vec<i32> = (0..run_len).map(|i| i % 10).collect();

    let data: Vec<String> = std::iter::once("i32".to_string()) // header
        .chain(i32_data.iter().map(|v| v.to_string()))
        .collect();
    let data = make_parquet(
        &data.join("\n"),
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i32", Type::INT32)])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "i32" => i32_data,
    )?;
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn many_runs_many_pages() -> Result<()> {
    let run_len = 10000;
    let i32_data: Vec<i32> = (0..run_len).map(|i| i % 10).collect();

    let data: Vec<String> = std::iter::once("i32".to_string()) // header
        .chain(i32_data.iter().map(|v| v.to_string()))
        .collect();
    let data = make_parquet(
        &data.join("\n"),
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(1000),
        Some(1000),
        Some(HashMap::from([("i32", Type::INT32)])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "i32" => i32_data,
    )?;
    assert_eq!(df, expected);

    Ok(())
}
