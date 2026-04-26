use anyhow::Result;
use insta::assert_snapshot;
use parquet_parser::reader::read_parquet;

use crate::make_parquet_file;

#[test]
fn boolean_column() -> Result<()> {
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
        &[],
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
fn boolean_column_all_true_false() -> Result<()> {
    let parquet_file = make_parquet_file(
        r#"
boolean_true,boolean_false
true,false
true,false
true,false
true,false
true,false
true,false
true,false
true,false
true,false
true,false
"#,
        &[],
    )?;

    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (10, 2)
    ┌──────────────┬───────────────┐
    │ boolean_true ┆ boolean_false │
    │ ---          ┆ ---           │
    │ bool         ┆ bool          │
    ╞══════════════╪═══════════════╡
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    │ true         ┆ false         │
    └──────────────┴───────────────┘
    ");
    Ok(())
}

#[test]
fn boolean_column_long() -> Result<()> {
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
true
true
false
true
true
false
true
true
false
"#,
        &[],
    )?;
    unsafe { std::env::set_var("POLARS_FMT_MAX_ROWS", "100") };
    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (69, 1)
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
    │ true    │
    │ true    │
    │ false   │
    │ true    │
    │ true    │
    │ false   │
    │ true    │
    │ true    │
    │ false   │
    └─────────┘
    ");
    Ok(())
}
