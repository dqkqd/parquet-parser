use std::collections::HashMap;

use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{
    file_metadata::read_file_metadata,
    format::Type,
    row_group::{read_row_group, read_row_groups},
};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn one_group() -> Result<()> {
    let data = make_parquet(
        r#"
i32,i64,float,double,string
1,1,1.1,1.1,one
2,2,2.2,2.2,two
3,3,3.3,3.3,three
4,4,4.4,4.4,four
5,5,5.5,5.5,five
6,6,6.6,6.6,six
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(2),
        None,
        Some(HashMap::from([
            ("i32", Type::INT32),
            ("i64", Type::INT64),
            ("float", Type::FLOAT),
            ("double", Type::DOUBLE),
            ("string", Type::BYTE_ARRAY),
        ])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let row_group = &file_metadata.row_groups[0];
    let df = read_row_group(data, row_group)?;
    let expected = df!(
        "i32" => [1i32, 2i32, 3i32, 4i32, 5i32, 6i32],
        "i64" => [1i64, 2i64, 3i64, 4i64, 5i64, 6i64],
        "float" => [1.1f32, 2.2f32, 3.3f32, 4.4f32, 5.5f32, 6.6f32],
        "double" => [1.1f64, 2.2f64, 3.3f64, 4.4f64, 5.5f64, 6.6f64],
        "string" => ["one", "two", "three", "four", "five", "six"],
    )?;
    assert_eq!(df, expected);

    Ok(())
}

#[test]
fn many_groups() -> Result<()> {
    let data = make_parquet(
        r#"
i32,i64,float,double,string
1,1,1.1,1.1,one
2,2,2.2,2.2,two
3,3,3.3,3.3,three
4,4,4.4,4.4,four
5,5,5.5,5.5,five
6,6,6.6,6.6,six
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(2),
        Some(2),
        Some(HashMap::from([
            ("i32", Type::INT32),
            ("i64", Type::INT64),
            ("float", Type::FLOAT),
            ("double", Type::DOUBLE),
            ("string", Type::BYTE_ARRAY),
        ])),
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "i32" => [1i32, 2i32, 3i32, 4i32, 5i32, 6i32],
        "i64" => [1i64, 2i64, 3i64, 4i64, 5i64, 6i64],
        "float" => [1.1f32, 2.2f32, 3.3f32, 4.4f32, 5.5f32, 6.6f32],
        "double" => [1.1f64, 2.2f64, 3.3f64, 4.4f64, 5.5f64, 6.6f64],
        "string" => ["one", "two", "three", "four", "five", "six"],
    )?;
    assert_eq!(df, expected);

    Ok(())
}
