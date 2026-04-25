use anyhow::Result;
use parquet::{
    basic::{Compression, Encoding},
    data_type::AsBytes,
};
use parquet_parser::{
    data_page::read_column_data_pages, file_metadata::read_file_metadata, format::PageType,
};

use crate::make_parquet;

#[test]
fn column_contains_one_page() -> Result<()> {
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

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;

    // no dictionary data pages
    assert!(column_data_pages.dictionary_page.is_none());

    assert_eq!(column_data_pages.data_pages.len(), 1);

    // header
    assert_eq!(
        column_data_pages.data_pages[0].page_header.type_,
        PageType::DATA_PAGE
    );

    // definition_levels
    assert!(column_data_pages.data_pages[0].definition_levels.is_some());
    assert_eq!(
        column_data_pages.data_pages[0]
            .definition_levels
            .as_ref()
            .unwrap()
            .as_ref(),
        [6, 1]
    );

    // encoded values
    assert_eq!(
        column_data_pages.data_pages[0].encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes(), 3i64.as_bytes()].concat()
    );

    Ok(())
}

#[test]
fn column_contains_many_pages() -> Result<()> {
    // create a parquet data with two pages
    let parquet_data = make_parquet(
        r#"
my_col
1
2
3
4
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
        Some(2),
        None,
        None,
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let column_data_pages = read_column_data_pages(parquet_data, column_metadata)?;

    // no dictionary data pages
    assert!(column_data_pages.dictionary_page.is_none());

    // first page
    assert_eq!(
        column_data_pages.data_pages[0].encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes()].concat()
    );

    // second page
    assert_eq!(
        column_data_pages.data_pages[1].encoded_values.as_ref(),
        [3i64.as_bytes(), 4i64.as_bytes()].concat()
    );

    Ok(())
}
