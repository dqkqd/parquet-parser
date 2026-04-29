use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::decoder::rle::rle_decode;
use parquet_parser::format::Type;
use polars::prelude::*;

#[test]
fn ok_true() -> Result<()> {
    let data = Bytes::from(0b00000001u8.as_bytes());
    let scalars = rle_decode(data, Type::BOOLEAN, 1, 3)?;
    assert_eq!(
        scalars,
        [Scalar::from(true), Scalar::from(true), Scalar::from(true),]
    );
    Ok(())
}

#[test]
fn ok_false() -> Result<()> {
    let data = Bytes::from(0b00000010u8.as_bytes());
    let scalars = rle_decode(data, Type::BOOLEAN, 1, 3)?;
    assert_eq!(
        scalars,
        [
            Scalar::from(false),
            Scalar::from(false),
            Scalar::from(false),
        ]
    );
    Ok(())
}
