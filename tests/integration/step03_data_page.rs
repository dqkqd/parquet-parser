use anyhow::Result;
use parquet::data_type::AsBytes;
use parquet_parser::{
    data_page::read_data_page, file_metadata::read_file_metadata, format::PageType,
};

use crate::make_parquet_bytes;

#[test]
fn read_one_data_page() -> Result<()> {
    let parquet_data = make_parquet_bytes(
        r#"
col1
1
2
3
"#,
        &[],
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let offset = column_metadata.data_page_offset as usize;
    let length = column_metadata.total_compressed_size as usize;
    let data_page_bytes = parquet_data.slice(offset..offset + length);

    let (data_page, _) = read_data_page(data_page_bytes, column_metadata.codec)?;

    // header
    assert_eq!(data_page.page_header.type_, PageType::DATA_PAGE);
    assert_eq!(data_page.num_values(), 3);

    // definition_levels
    assert!(data_page.definition_levels.is_some());
    assert_eq!(data_page.definition_levels.unwrap().as_ref(), [6, 1]);

    // encoded values
    assert_eq!(
        data_page.encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes(), 3i64.as_bytes()].concat()
    );

    Ok(())
}

#[test]
fn remaining_bytes_must_be_correct() -> Result<()> {
    // create a parquet data with two pages
    let parquet_data = make_parquet_bytes(
        r#"
col1
1
2
3
"#,
        &[&["--rows-per-page", "2"]],
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let offset = column_metadata.data_page_offset as usize;
    let length = column_metadata.total_compressed_size as usize;
    let data_page_bytes = parquet_data.slice(offset..offset + length);

    // first page
    let (data_page, remaining_bytes) = read_data_page(data_page_bytes, column_metadata.codec)?;
    assert_eq!(data_page.num_values(), 2);
    assert_eq!(
        data_page.encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes()].concat()
    );

    // second page
    let (data_page, remaining_bytes) = read_data_page(remaining_bytes, column_metadata.codec)?;
    assert_eq!(data_page.num_values(), 1);
    assert_eq!(
        data_page.encoded_values.as_ref(),
        [3i64.as_bytes()].concat()
    );

    // there is no remaining page data!
    assert!(remaining_bytes.is_empty());

    Ok(())
}
