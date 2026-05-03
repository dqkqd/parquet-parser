use anyhow::Result;
use insta::assert_snapshot;
use parquet_parser::reader::read_parquet;

use crate::make_parquet_file;

#[test]
fn nulls_no_dictionary() -> Result<()> {
    let parquet_file = make_parquet_file(
        r#"
my_col
one
two
""
""
three
"#,
        &[],
    )?;

    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (5, 1)
    ┌────────┐
    │ my_col │
    │ ---    │
    │ str    │
    ╞════════╡
    │ one    │
    │ two    │
    │ null   │
    │ null   │
    │ three  │
    └────────┘
    ");
    Ok(())
}

#[test]
fn nulls_dictionary() -> Result<()> {
    let parquet_file = make_parquet_file(
        r#"
my_col
one
two
""
""
three
"#,
        &[&["--dictionary"]],
    )?;

    assert_snapshot!(read_parquet(parquet_file)?, @"
    shape: (5, 1)
    ┌────────┐
    │ my_col │
    │ ---    │
    │ str    │
    ╞════════╡
    │ one    │
    │ two    │
    │ null   │
    │ null   │
    │ three  │
    └────────┘
    ");
    Ok(())
}
