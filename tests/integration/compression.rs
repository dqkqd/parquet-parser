use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::file_metadata::read_file_metadata;
use polars::prelude::*;
use rstest::rstest;

#[rstest]
#[case::uncompress(Compression::UNCOMPRESSED)]
#[case::snappy(Compression::SNAPPY)]
fn ok(#[case] compression: Compression) -> Result<()> {
    use parquet_parser::row_group::read_row_groups;

    use crate::make_parquet;

    let data = make_parquet(
        r#"
boolean_col,i64_col,double_col,string_col
true,1,1.1,one
true,2,2.2,two
false,3,3.3,three
true,4,4.4,four
false,5,5.5,five
false,6,6.6,six
"#,
        false,
        Encoding::PLAIN,
        compression,
        Some(2),
        Some(2),
        None,
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "boolean_col" => [true, true, false, true, false, false],
        "i64_col" => [1i64, 2i64, 3i64, 4i64, 5i64, 6i64],
        "double_col" => [1.1, 2.2, 3.3, 4.4, 5.5, 6.6],
        "string_col" => ["one", "two", "three", "four", "five", "six"],
    )?;
    assert_eq!(df, expected);

    Ok(())
}
