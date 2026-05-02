use anyhow::Result;
use parquet_parser::{
    decoder::decode_page,
    file_metadata::read_file_metadata,
    page::{Page, read_pages},
};
use polars::prelude::*;

use crate::make_parquet_bytes;

#[test]
fn dictionary_enabled() -> Result<()> {
    let parquet_data = make_parquet_bytes(
        r#"
my_col
one
two
three
one
two
three
"#,
        &[&["--dictionary"]],
    )?;

    let file_metadata = read_file_metadata(parquet_data.clone())?;
    let column_metadata = file_metadata.row_groups[0].columns[0]
        .meta_data
        .as_ref()
        .unwrap();

    let mut pages = read_pages(parquet_data.clone(), column_metadata)?;

    // the first page is the dictionary, the second page is the data page
    assert_eq!(pages.len(), 2);
    assert!(matches!(pages[0], Page::DictionaryPage { .. }));
    assert!(matches!(pages[1], Page::DataPage { .. }));

    let _ = pages.pop().unwrap();
    let dictionary_page = pages.pop().unwrap();

    // the dictionary page contains unique value and is encoded using plain encoding!
    assert_eq!(dictionary_page.num_values(), 3);
    assert_eq!(
        decode_page(dictionary_page, column_metadata.type_, 3)?,
        [
            Scalar::from(PlSmallStr::from("one")),
            Scalar::from(PlSmallStr::from("two")),
            Scalar::from(PlSmallStr::from("three")),
        ]
    );

    Ok(())
}
