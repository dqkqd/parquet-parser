use anyhow::Result;
use parquet::{
    basic::{Compression, Encoding},
    data_type::AsBytes,
};
use parquet_parser::{data_page::read_data_pages, file_metadata::read_file_metadata};

use crate::make_parquet;

#[test]
fn one_page() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
my_col
1
2
3
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        None,
        None,
        None,
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let i64_column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let data_pages = read_data_pages(parquet_data.clone(), i64_column_metadata)?;
    assert_eq!(data_pages.data_pages.len(), 1);
    assert_eq!(data_pages.data_pages[0].num_values(), 3);
    assert_eq!(
        data_pages.data_pages[0].encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes(), 3i64.as_bytes()].concat()
    );

    Ok(())
}

#[test]
fn many_pages() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
my_col
1
2
3
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(2),
        None,
        None,
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let i64_column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let data_pages = read_data_pages(parquet_data.clone(), i64_column_metadata)?;
    assert_eq!(data_pages.data_pages.len(), 2);

    assert_eq!(data_pages.data_pages[0].num_values(), 2);
    assert_eq!(
        data_pages.data_pages[0].encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes()].concat()
    );

    assert_eq!(data_pages.data_pages[1].num_values(), 1);
    assert_eq!(
        data_pages.data_pages[1].encoded_values.as_ref(),
        3i64.as_bytes()
    );

    Ok(())
}
