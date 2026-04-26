use std::collections::HashMap;

use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{
    data_page::read_column_data_pages, decoder::decode_data_page,
    file_metadata::read_file_metadata, format::Type,
};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn i32_ok() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
i32
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i32", Type::INT32)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    let decoded = decode_data_page(data_page, Type::INT32, 3)?;
    assert_eq!(
        decoded,
        [
            Scalar::from(10i32),
            Scalar::from(20i32),
            Scalar::from(30i32)
        ]
    );

    Ok(())
}

#[test]
fn i32_too_many_values() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
i32
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i32", Type::INT32)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    assert!(decode_data_page(data_page, Type::INT32, 4).is_err());

    Ok(())
}

#[test]
fn i64_ok() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
i64
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i64", Type::INT64)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    let decoded = decode_data_page(data_page, Type::INT64, 3)?;
    assert_eq!(
        decoded,
        [
            Scalar::from(10i64),
            Scalar::from(20i64),
            Scalar::from(30i64)
        ]
    );

    Ok(())
}

#[test]
fn i64_too_many_values() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
i64
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("i64", Type::INT64)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    assert!(decode_data_page(data_page, Type::INT64, 4).is_err());

    Ok(())
}

#[test]
fn float_ok() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
float
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("float", Type::FLOAT)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    let decoded = decode_data_page(data_page, Type::FLOAT, 3)?;
    assert_eq!(
        decoded,
        [
            Scalar::from(10f32),
            Scalar::from(20f32),
            Scalar::from(30f32)
        ]
    );

    Ok(())
}

#[test]
fn float_too_many_values() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
float
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("float", Type::FLOAT)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    assert!(decode_data_page(data_page, Type::FLOAT, 4).is_err());

    Ok(())
}

#[test]
fn double_ok() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
double
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("double", Type::DOUBLE)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    let decoded = decode_data_page(data_page, Type::DOUBLE, 3)?;
    assert_eq!(
        decoded,
        [
            Scalar::from(10f64),
            Scalar::from(20f64),
            Scalar::from(30f64)
        ]
    );

    Ok(())
}

#[test]
fn double_too_many_values() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
double
10
20
30
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("double", Type::DOUBLE)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    assert!(decode_data_page(data_page, Type::DOUBLE, 4).is_err());

    Ok(())
}

#[test]
fn string_ok() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
string
one
two
three
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("string", Type::BYTE_ARRAY)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    let decoded = decode_data_page(data_page, Type::BYTE_ARRAY, 3)?;
    assert_eq!(
        decoded,
        [
            Scalar::from(PlSmallStr::from_static("one")),
            Scalar::from(PlSmallStr::from_static("two")),
            Scalar::from(PlSmallStr::from_static("three")),
        ]
    );

    Ok(())
}

#[test]
fn string_too_many_values() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
string
one
two
three
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        Some(HashMap::from([("string", Type::BYTE_ARRAY)])),
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let mut column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;
    let data_page = column_data_pages.data_pages.pop().unwrap();

    assert!(decode_data_page(data_page, Type::BYTE_ARRAY, 4).is_err());

    Ok(())
}
