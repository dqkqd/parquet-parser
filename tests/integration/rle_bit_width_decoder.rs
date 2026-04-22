use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{decoder::rle::rle_decode, format::Type};
use polars::prelude::*;

#[test]
fn bit_width_32() -> Result<()> {
    let mut data = Bytes::from(123i32.as_bytes());
    assert_eq!(data.len(), 4);
    assert_eq!(
        rle_decode(&mut data, Type::INT32, 32, 3)?,
        [Scalar::from(123), Scalar::from(123), Scalar::from(123),]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn bit_width_3() -> Result<()> {
    let mut data = Bytes::from(0b000_001_010_011_100_101_110_111i32.as_bytes());
    assert_eq!(data.len(), 4);
    assert_eq!(
        rle_decode(&mut data, Type::INT32, 3, 3)?,
        [Scalar::from(7), Scalar::from(7), Scalar::from(7),]
    );
    assert_eq!(data.len(), 3);
    Ok(())
}

#[test]
fn bit_width_1() -> Result<()> {
    let mut data = Bytes::from(0b0011u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert_eq!(
        rle_decode(&mut data, Type::INT32, 1, 4)?,
        [
            Scalar::from(1),
            Scalar::from(1),
            Scalar::from(1),
            Scalar::from(1),
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn too_many_values() -> Result<()> {
    let mut data = Bytes::new();
    assert_eq!(data.len(), 0);
    assert!(rle_decode(&mut data, Type::INT32, 32, 10).is_err());
    Ok(())
}

#[test]
fn not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(1u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert!(rle_decode(&mut data, Type::INT32, 10, 1).is_err());
    Ok(())
}
