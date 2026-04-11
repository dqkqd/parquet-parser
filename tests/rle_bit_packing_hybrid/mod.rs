use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{file_metadata::read_file_metadata, row_group::read_row_groups};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn mixed() -> Result<()> {
    let rle_run_len = 5000;
    let bit_packing_run_len = 3456;

    let rle_run: Vec<bool> = (0..rle_run_len).map(|i| i < rle_run_len / 2).collect();
    let bit_packed_run: Vec<bool> = (0..bit_packing_run_len).map(|i| i % 2 == 0).collect();

    let boolean_data: Vec<bool> = [
        rle_run.clone(),
        bit_packed_run.clone(),
        rle_run.clone(),
        bit_packed_run.clone(),
    ]
    .concat();

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
