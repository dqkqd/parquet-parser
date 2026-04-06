use std::collections::HashMap;

use anyhow::Result;
use bytes::Bytes;
use parquet::{
    basic::{Compression, Encoding},
    data_type::AsBytes,
};
use parquet_parser::{
    decoder::bit_packed::bit_packed_decode, file_metadata::read_file_metadata, format::Type,
    row_group::read_row_groups,
};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn div_by_8() -> Result<()> {
    let mut data = Bytes::from(0b1101100011011000u16.as_bytes());
    assert_eq!(data.len(), 2);
    assert_eq!(
        bit_packed_decode(&mut data, Type::BOOLEAN, 1, 16)?,
        [
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true)
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn not_div_by_8() -> Result<()> {
    let mut data = Bytes::from(0b1101100011011000u16.as_bytes());
    assert_eq!(data.len(), 2);
    assert_eq!(
        bit_packed_decode(&mut data, Type::BOOLEAN, 1, 13)?,
        [
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(true),
            Scalar::from(true)
        ],
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(0b1101100011011000u16.as_bytes());
    assert_eq!(data.len(), 2);
    assert!(bit_packed_decode(&mut data, Type::BOOLEAN, 1, 18).is_err());
    Ok(())
}

#[test]
fn boolean_column() -> Result<()> {
    let data = make_parquet(
        r#"
boolean
true
true
true
false
false
true
false
false
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(2),
        Some(2),
        Some(HashMap::from([("boolean", Type::BOOLEAN)])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "boolean" => [true, true, true, false, false, true, false, false],
    )?;
    assert_eq!(df, expected);

    Ok(())
}
