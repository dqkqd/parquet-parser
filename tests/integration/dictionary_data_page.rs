use anyhow::Result;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{
    data_page::{decode_data_page, read_data_pages},
    file_metadata::read_file_metadata,
};
use polars::prelude::*;

use crate::make_parquet;

#[test]
fn disabled() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
my_col
one
two
one
two
three
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
    let data_pages = read_data_pages(parquet_data.clone(), column_metadata)?;
    assert!(data_pages.dictionary_page.is_none());
    assert_eq!(data_pages.data_pages.len(), 3);

    Ok(())
}

#[test]
fn enabled() -> Result<()> {
    let parquet_data = make_parquet(
        r#"
my_col
one
two
one
two
three
"#,
        true,
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
    let data_pages = read_data_pages(parquet_data.clone(), column_metadata)?;
    assert!(data_pages.dictionary_page.is_some());
    assert_eq!(data_pages.data_pages.len(), 3);

    let dictionary_page = data_pages.dictionary_page.unwrap();

    let num_values = dictionary_page.num_values() as usize; // TODO: cleanup
    let dictionary_values = decode_data_page(dictionary_page, column_metadata.type_, num_values)?;
    assert_eq!(
        dictionary_values,
        [
            Scalar::from(PlSmallStr::from("one")),
            Scalar::from(PlSmallStr::from("two")),
            Scalar::from(PlSmallStr::from("three")),
        ]
    );

    Ok(())
}
