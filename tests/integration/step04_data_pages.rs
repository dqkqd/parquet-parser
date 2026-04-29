use anyhow::Result;
use parquet::data_type::AsBytes;
use parquet_parser::{
    file_metadata::read_file_metadata,
    page::{Page, read_pages},
};

use crate::make_parquet_bytes;

#[test]
fn column_contains_one_page() -> Result<()> {
    let parquet_data = make_parquet_bytes(
        r#"
my_col
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
    let mut pages = read_pages(parquet_data, column_metadata)?;

    assert_eq!(pages.len(), 1);
    let page = pages.pop().unwrap();

    // header
    let Page::DataPage {
        definition_levels,
        encoded_values,
        ..
    } = page
    else {
        panic!("expect data page");
    };

    // definition_levels
    assert_eq!(definition_levels.as_ref(), [6, 1]);

    // encoded values
    assert_eq!(
        encoded_values.as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes(), 3i64.as_bytes()].concat()
    );

    Ok(())
}

#[test]
fn column_contains_many_pages() -> Result<()> {
    // create a parquet data with two pages
    let parquet_data = make_parquet_bytes(
        r#"
my_col
1
2
3
4
"#,
        &[&["--rows-per-page", "2"]],
    )?;
    let file_metadata = read_file_metadata(parquet_data.clone())?;

    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();
    let pages = read_pages(parquet_data, column_metadata)?;

    // first page
    assert_eq!(
        pages[0].encoded_values().as_ref(),
        [1i64.as_bytes(), 2i64.as_bytes()].concat()
    );

    // second page
    assert_eq!(
        pages[1].encoded_values().as_ref(),
        [3i64.as_bytes(), 4i64.as_bytes()].concat()
    );

    Ok(())
}
