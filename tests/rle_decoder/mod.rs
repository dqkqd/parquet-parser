use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{
    decoder::{Decode, rle::RleDecoder},
    format::Type,
};
use polars::prelude::*;

#[test]
fn boolean() -> Result<()> {
    let mut data = Bytes::from(1u8.as_bytes());
    assert_eq!(data.len(), 1);
    let decoder = RleDecoder::new(1, Type::BOOLEAN);
    assert_eq!(
        decoder.decode(&mut data, 3)?,
        [Scalar::from(true), Scalar::from(true), Scalar::from(true)]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn i64() -> Result<()> {
    let mut data = Bytes::from(1234i64.as_bytes());
    assert_eq!(data.len(), 8);
    let decoder = RleDecoder::new(64, Type::INT64);
    assert_eq!(
        decoder.decode(&mut data, 3)?,
        [
            Scalar::from(1234i64),
            Scalar::from(1234i64),
            Scalar::from(1234i64),
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn double() -> Result<()> {
    let mut data = Bytes::from(1.234f64.as_bytes());
    assert_eq!(data.len(), 8);
    let decoder = RleDecoder::new(64, Type::DOUBLE);
    assert_eq!(
        decoder.decode(&mut data, 3)?,
        [
            Scalar::from(1.234f64),
            Scalar::from(1.234f64),
            Scalar::from(1.234f64),
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}
