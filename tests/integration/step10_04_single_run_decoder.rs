use anyhow::Result;
use insta::assert_snapshot;
use parquet_parser::reader::read_parquet;

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
