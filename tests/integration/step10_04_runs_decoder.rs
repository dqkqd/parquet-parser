use anyhow::Result;
use insta::assert_snapshot;
use parquet_parser::reader::read_parquet;
use polars::prelude::*;

use crate::make_parquet_file;

#[test]
fn bit_packed_single_run() -> Result<()> {
    let parquet_file = make_parquet_file(
        r#"
boolean
true
true
false
true
true
false
true
true
false
true
"#,
        &[&["--encoding", "rle"]],
    )?;

    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (10, 1)
    ┌─────────┐
    │ boolean │
    │ ---     │
    │ bool    │
    ╞═════════╡
    │ true    │
    │ true    │
    │ false   │
    │ true    │
    │ true    │
    │ false   │
    │ true    │
    │ true    │
    │ false   │
    │ true    │
    └─────────┘
    ");
    Ok(())
}

#[test]
fn rle_single_run() -> Result<()> {
    let parquet_file = make_parquet_file(
        r#"
boolean
true
true
true
true
true
true
true
true
true
true
"#,
        &[&["--encoding", "rle"]],
    )?;

    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (10, 1)
    ┌─────────┐
    │ boolean │
    │ ---     │
    │ bool    │
    ╞═════════╡
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    │ true    │
    └─────────┘
    ");
    Ok(())
}

#[test]
fn bit_packed_many_runs() -> Result<()> {
    let length = 1000;
    // the first 8 values are true, then false, then true, etc.
    let data: Vec<bool> = (0..length).map(|v| v % 2 == 0).collect();

    let parquet_data: Vec<&str> = vec!["boolean"]
        .into_iter()
        .chain(data.iter().map(|v| if *v { "true" } else { "false" }))
        .collect();

    let parquet_file = make_parquet_file(&parquet_data.join("\n"), &[&["--encoding", "rle"]])?;
    let df = read_parquet(parquet_file)?;
    assert_eq!(df.height(), length);

    let column = df.column("boolean")?;
    for (i, expected) in data.iter().enumerate() {
        let value = column.get(i)?;
        assert_eq!(value, AnyValue::Boolean(*expected));
    }

    Ok(())
}

#[test]
fn rle_many_runs() -> Result<()> {
    let length = 100;
    // the first 8 values are true, then false, then true, etc.
    let data: Vec<bool> = (0..length).map(|v| (v / 8) % 2 == 0).collect();

    let parquet_data: Vec<&str> = vec!["boolean"]
        .into_iter()
        .chain(data.iter().map(|v| if *v { "true" } else { "false" }))
        .collect();

    let parquet_file = make_parquet_file(&parquet_data.join("\n"), &[&["--encoding", "rle"]])?;
    let df = read_parquet(parquet_file)?;
    assert_eq!(df.height(), length);

    let column = df.column("boolean")?;
    for (i, expected) in data.iter().enumerate() {
        let value = column.get(i)?;
        assert_eq!(value, AnyValue::Boolean(*expected));
    }

    Ok(())
}
