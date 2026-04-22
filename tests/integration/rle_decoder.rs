use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{decoder::rle::rle_decode, format::Type};
use polars::prelude::*;

#[test]
fn ok() -> Result<()> {
    let mut data = Bytes::from(1u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert_eq!(
        rle_decode(&mut data, Type::BOOLEAN, 1, 3)?,
        vec![Scalar::from(true); 3]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn not_enough_bytes() -> Result<()> {
    let mut data = Bytes::new();
    assert!(rle_decode(&mut data, Type::BOOLEAN, 1, 3).is_err());
    Ok(())
}
