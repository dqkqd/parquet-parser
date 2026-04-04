use anyhow::Result;
use bytes::Bytes;
use parquet::data_type::AsBytes;
use parquet_parser::{
    decoder::{Decode, PlainDecoder},
    format::Type,
};
use polars::prelude::*;

#[test]
fn i64_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            1i64.as_bytes(),                  // 8 bytes
            123456789101112131i64.as_bytes(), // 8 bytes
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::INT64);
    assert_eq!(
        decoder.decode(&mut data, 2)?,
        [Scalar::from(1i64), Scalar::from(123456789101112131i64),]
    );
    Ok(())
}

#[test]
fn i64_too_many_values() -> Result<()> {
    let mut data = Bytes::from(
        [
            1i64.to_le_bytes().as_slice(), // 8 bytes
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::INT64);
    assert!(decoder.decode(&mut data, 2).is_err());
    Ok(())
}

#[test]
fn i64_not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            1i64.to_le_bytes().as_slice(), // 8 bytes
            1i32.to_le_bytes().as_slice(), // 4 bytes: this causes error
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::INT64);
    assert!(decoder.decode(&mut data, 2).is_err());
    Ok(())
}

#[test]
fn double_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            1.0f64.to_le_bytes().as_slice(), // 8 bytes
            2.2f64.to_le_bytes().as_slice(), // 8 bytes
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::DOUBLE);
    assert_eq!(
        decoder.decode(&mut data, 2)?,
        [Scalar::from(1.0), Scalar::from(2.2),]
    );
    Ok(())
}

#[test]
fn double_not_enough_length() -> Result<()> {
    let mut data = Bytes::from(
        [
            1.0f64.to_le_bytes().as_slice(), // 8 bytes
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::DOUBLE);
    assert!(decoder.decode(&mut data, 2).is_err());
    Ok(())
}

#[test]
fn double_not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            1.0f64.to_le_bytes().as_slice(), // 8 bytes
            2.3f32.to_le_bytes().as_slice(), // 4 bytes: this causes error
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::DOUBLE);
    assert!(decoder.decode(&mut data, 2).is_err());
    Ok(())
}

#[test]
fn string_ok() -> Result<()> {
    let mut data = Bytes::from(
        [
            5i32.to_le_bytes().as_slice(),
            b"Hello".as_slice(), // Hello
            6i32.to_le_bytes().as_slice(),
            b"World!".as_slice(), // World!
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::BYTE_ARRAY);
    assert_eq!(
        decoder.decode(&mut data, 2)?,
        [
            Scalar::from(PlSmallStr::from_static("Hello")),
            Scalar::from(PlSmallStr::from_static("World!")),
        ]
    );
    Ok(())
}

#[test]
fn string_not_enough_length() -> Result<()> {
    let mut data = Bytes::from(
        [
            5i32.to_le_bytes().as_slice(),
            b"Hello".as_slice(), // Hello
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::BYTE_ARRAY);
    assert!(decoder.decode(&mut data, 2).is_err());
    Ok(())
}

#[test]
fn string_not_enough_bytes() -> Result<()> {
    let mut data = Bytes::from(
        [
            8i32.to_le_bytes().as_slice(),
            b"Missing".as_slice(), // Missing 1 byte, it should be error
        ]
        .concat(),
    );
    let decoder = PlainDecoder::new(Type::BYTE_ARRAY);
    assert!(decoder.decode(&mut data, 1).is_err());
    Ok(())
}
