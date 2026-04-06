use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{columns::read_column, file_metadata::read_file_metadata};
use polars::prelude::Column;

use crate::make_parquet;

#[test]
fn simple_i64() -> Result<()> {
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
    )?;

    let file_metadata = read_file_metadata(data.clone())?;

    let column_chunk = &file_metadata.row_groups[0].columns[0];
    let column = read_column(data.clone(), column_chunk)?;
    let expected = Column::new(
        "my_col".into(),
        [Some(1i64), Some(2i64), None, None, Some(5i64), Some(6i64)],
    );
    assert_eq!(column.name(), expected.name());
    assert_eq!(column, expected);

    Ok(())
}
