use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{decoder::plain::plain_decode, format::Type};
use polars::prelude::*;

#[test]
fn i32_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            1234567i32.as_bytes(),   // 4 bytes
            789101112i32.as_bytes(), // 4 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 8);
    assert_eq!(
        plain_decode(&mut data, Type::INT32, 2)?,
        [Scalar::from(1234567i32), Scalar::from(789101112i32),]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn i32_too_many_values() -> Result<()> {
    let mut data = Bytes::from(
        [
            1234567i32.as_bytes(),   // 4 bytes
            789101112i32.as_bytes(), // 4 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 8);
    assert!(plain_decode(&mut data, Type::INT32, 4).is_err());
    Ok(())
}

#[test]
fn i32_missing_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            123u8.as_bytes(), // 1 byte
        ]
        .concat(),
    );
    assert_eq!(data.len(), 1);
    assert!(plain_decode(&mut data, Type::INT32, 1).is_err());
    Ok(())
}

#[test]
fn i64_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            1234567i64.as_bytes(),   // 8 bytes
            789101112i64.as_bytes(), // 8 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 16);
    assert_eq!(
        plain_decode(&mut data, Type::INT64, 2)?,
        [Scalar::from(1234567i64), Scalar::from(789101112i64),]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn i64_too_many_values() -> Result<()> {
    let mut data = Bytes::from(
        [
            1234567i64.as_bytes(),   // 8 bytes
            789101112i64.as_bytes(), // 8 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 16);
    assert!(plain_decode(&mut data, Type::INT64, 4).is_err());
    Ok(())
}

#[test]
fn i64_missing_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            123u8.as_bytes(), // 1 byte
        ]
        .concat(),
    );
    assert_eq!(data.len(), 1);
    assert!(plain_decode(&mut data, Type::INT64, 1).is_err());
    Ok(())
}

#[test]
fn float_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            123.4567f32.as_bytes(),   // 4 bytes
            789.101_14_f32.as_bytes(), // 4 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 8);
    assert_eq!(
        plain_decode(&mut data, Type::FLOAT, 2)?,
        [Scalar::from(123.4567f32), Scalar::from(789.101_14_f32),]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn float_too_many_values() -> Result<()> {
    let mut data = Bytes::from(
        [
            123.4567f32.as_bytes(),   // 4 bytes
            789.101_14_f32.as_bytes(), // 4 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 8);
    assert!(plain_decode(&mut data, Type::FLOAT, 4).is_err());
    Ok(())
}

#[test]
fn float_missing_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            123u8.as_bytes(), // 1 byte
        ]
        .concat(),
    );
    assert_eq!(data.len(), 1);
    assert!(plain_decode(&mut data, Type::FLOAT, 1).is_err());
    Ok(())
}

#[test]
fn double_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            123.4567f64.as_bytes(),   // 8 bytes
            789.101112f64.as_bytes(), // 8 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 16);
    assert_eq!(
        plain_decode(&mut data, Type::DOUBLE, 2)?,
        [Scalar::from(123.4567f64), Scalar::from(789.101112f64),]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn double_too_many_values() -> Result<()> {
    let mut data = Bytes::from(
        [
            123.4567f64.as_bytes(),   // 8 bytes
            789.101112f64.as_bytes(), // 8 bytes
        ]
        .concat(),
    );
    assert_eq!(data.len(), 16);
    assert!(plain_decode(&mut data, Type::DOUBLE, 4).is_err());
    Ok(())
}

#[test]
fn double_missing_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            123u8.as_bytes(), // 1 byte
        ]
        .concat(),
    );
    assert_eq!(data.len(), 1);
    assert!(plain_decode(&mut data, Type::DOUBLE, 1).is_err());
    Ok(())
}

#[test]
fn string_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            5i32.to_le_bytes().as_slice(),
            b"Hello".as_slice(),
            6i32.to_le_bytes().as_slice(),
            b"World!".as_slice(),
        ]
        .concat(),
    );
    assert_eq!(data.len(), 19);
    assert_eq!(
        plain_decode(&mut data, Type::BYTE_ARRAY, 2)?,
        [
            Scalar::from(PlSmallStr::from_static("Hello")),
            Scalar::from(PlSmallStr::from_static("World!")),
        ]
    );
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn string_too_many_values() -> Result<()> {
    let mut data = Bytes::from([5i32.to_le_bytes().as_slice(), b"Hello".as_slice()].concat());
    assert!(plain_decode(&mut data, Type::BYTE_ARRAY, 2).is_err());
    Ok(())
}

#[test]
fn string_string_too_short() -> Result<()> {
    let mut data = Bytes::from(
        [
            8i32.to_le_bytes().as_slice(),
            b"Missing".as_slice(), // Missing 1 byte, it should be error
        ]
        .concat(),
    );
    assert!(plain_decode(&mut data, Type::BYTE_ARRAY, 1).is_err());
    Ok(())
}
