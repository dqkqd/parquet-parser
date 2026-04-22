use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{file_metadata::read_file_metadata, row_group::read_row_groups};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn null_no_dictionary() -> Result<()> {
    let data = make_parquet(
        r#"
my_col
1
2
""
""
5
6
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        None,
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let df = read_row_groups(data, &file_metadata.row_groups)?;
    let expected = df!(
        "my_col" => [Some(1), Some(2), None, None, Some(5), Some(6)],
    )?;
    assert_eq!(df, expected);

    Ok(())
}
