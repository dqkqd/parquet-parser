use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{decoder::bit_packed::bit_packed_decode, format::Type};
use polars::prelude::*;

#[test]
fn bit_width_32() -> Result<()> {
    let mut data = Bytes::from([123i32.as_bytes(), 456i32.as_bytes(), 789i32.as_bytes()].concat());
    assert_eq!(data.len(), 12);
    assert_eq!(
        bit_packed_decode(&mut data, Type::INT32, 32, 3)?,
        [Scalar::from(123), Scalar::from(456), Scalar::from(789)]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn bit_width_3() -> Result<()> {
    let mut data = Bytes::from(0b000_001_010_011_100_101_110_111i32.as_bytes());
    assert_eq!(data.len(), 4);
    assert_eq!(
        bit_packed_decode(&mut data, Type::INT32, 3, 8)?,
        [
            Scalar::from(7),
            Scalar::from(6),
            Scalar::from(5),
            Scalar::from(4),
            Scalar::from(3),
            Scalar::from(2),
            Scalar::from(1),
            Scalar::from(0)
        ]
    );
    assert_eq!(data.len(), 1);
    Ok(())
}

#[test]
fn bit_width_1() -> Result<()> {
    let mut data = Bytes::from(0b1011u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert_eq!(
        bit_packed_decode(&mut data, Type::INT32, 1, 4)?,
        [
            Scalar::from(1),
            Scalar::from(1),
            Scalar::from(0),
            Scalar::from(1)
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn too_many_values() -> Result<()> {
    let mut data = Bytes::from(1u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert!(bit_packed_decode(&mut data, Type::INT32, 3, 10).is_err());
    Ok(())
}

#[test]
fn not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(1u8.as_bytes());
    assert_eq!(data.len(), 1);
    assert!(bit_packed_decode(&mut data, Type::INT32, 10, 1).is_err());
    Ok(())
}
