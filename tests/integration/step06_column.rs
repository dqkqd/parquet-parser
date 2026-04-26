use anyhow::Result;
use parquet_parser::{column::read_column, file_metadata::read_file_metadata};
use polars::prelude::*;

use crate::make_parquet_bytes;

#[test]
fn i32() -> Result<()> {
    let data = make_parquet_bytes(
        r#"
my_col
1
2
3
4
5
6
"#,
        &[&["--dtypes", "my_col=int32"]],
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new("my_col".into(), [1i32, 2i32, 3i32, 4i32, 5i32, 6i32]);
    assert_eq!(column.name(), expected.name());
    assert_eq!(column.dtype(), expected.dtype());
    assert_eq!(column, expected);

    Ok(())
}

#[test]
fn i64() -> Result<()> {
    let data = make_parquet_bytes(
        r#"
my_col
1
2
3
4
5
6
"#,
        &[&["--dtypes", "my_col=int64"]],
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new("my_col".into(), [1i64, 2i64, 3i64, 4i64, 5i64, 6i64]);
    assert_eq!(column.name(), expected.name());
    assert_eq!(column.dtype(), expected.dtype());
    assert_eq!(column, expected);

    Ok(())
}

#[test]
fn float() -> Result<()> {
    let data = make_parquet_bytes(
        r#"
my_col
1.1
2.2
3.3
4.4
5.5
6.6
"#,
        &[&["--dtypes", "my_col=float"]],
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new(
        "my_col".into(),
        [1.1f32, 2.2f32, 3.3f32, 4.4f32, 5.5f32, 6.6f32],
    );
    assert_eq!(column.name(), expected.name());
    assert_eq!(column.dtype(), expected.dtype());
    assert_eq!(column, expected);

    Ok(())
}

#[test]
fn double() -> Result<()> {
    let data = make_parquet_bytes(
        r#"
my_col
1.1
2.2
3.3
4.4
5.5
6.6
"#,
        &[&["--dtypes", "my_col=double"]],
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new(
        "my_col".into(),
        [1.1f64, 2.2f64, 3.3f64, 4.4f64, 5.5f64, 6.6f64],
    );
    assert_eq!(column.name(), expected.name());
    assert_eq!(column.dtype(), expected.dtype());
    assert_eq!(column, expected);

    Ok(())
}

#[test]
fn string() -> Result<()> {
    let data = make_parquet_bytes(
        r#"
my_col
one
two
three
four
five
six
"#,
        &[&["--dtypes", "my_col=string"]],
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new(
        "my_col".into(),
        ["one", "two", "three", "four", "five", "six"],
    );
    assert_eq!(column.name(), expected.name());
    assert_eq!(column.dtype(), expected.dtype());
    assert_eq!(column, expected);

    Ok(())
}
