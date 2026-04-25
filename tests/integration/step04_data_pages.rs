use anyhow::Result;
use parquet::{
    basic::{Compression, Encoding},
    data_type::AsBytes,
};
use parquet_parser::{
    data_page::read_data_page, file_metadata::read_file_metadata, format::PageType,
};

use crate::make_parquet;

#[test]
fn read_one_data_page() -> Result<()> {
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
    let data_page_bytes = parquet_data.slice(column_metadata.data_page_offset as usize..);
    let (data_page, _) = read_data_page(data_page_bytes, column_metadata.codec)?;

    // header
    assert_eq!(data_page.page_header.type_, PageType::DATA_PAGE);
    assert_eq!(data_page.num_values(), 3);

    // repetition must be none
    assert!(data_page.repetiion_levels.is_none());

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

// #[test]
// fn one_page() -> Result<()> {
//     let parquet_data = make_parquet(
//         r#"
// my_col
// 1
// 2
// 3
// "#,
//         false,
//         Encoding::PLAIN,
//         Compression::UNCOMPRESSED,
//         None,
//         None,
//         None,
//     )?;
//     let file_metadata = read_file_metadata(parquet_data.clone())?;
//
//     let i64_column_metadata = file_metadata.row_groups[0].columns[0]
//         .meta_data
//         .as_ref()
//         .unwrap();
//     let data_pages = read_data_pages(parquet_data.clone(), i64_column_metadata)?;
//     assert_eq!(data_pages.data_pages.len(), 1);
//     assert_eq!(data_pages.data_pages[0].num_values(), 3);
//     assert_eq!(
//         data_pages.data_pages[0].encoded_values.as_ref(),
//         [1i64.as_bytes(), 2i64.as_bytes(), 3i64.as_bytes()].concat()
//     );
//
//     Ok(())
// }
//
// #[test]
// fn many_pages() -> Result<()> {
//     let parquet_data = make_parquet(
//         r#"
// my_col
// 1
// 2
// 3
// "#,
//         false,
//         Encoding::PLAIN,
//         Compression::UNCOMPRESSED,
//         Some(2),
//         None,
//         None,
//     )?;
//     let file_metadata = read_file_metadata(parquet_data.clone())?;
//
//     let i64_column_metadata = file_metadata.row_groups[0].columns[0]
//         .meta_data
//         .as_ref()
//         .unwrap();
//     let data_pages = read_data_pages(parquet_data.clone(), i64_column_metadata)?;
//     assert_eq!(data_pages.data_pages.len(), 2);
//
//     assert_eq!(data_pages.data_pages[0].num_values(), 2);
//     assert_eq!(
//         data_pages.data_pages[0].encoded_values.as_ref(),
//         [1i64.as_bytes(), 2i64.as_bytes()].concat()
//     );
//
//     assert_eq!(data_pages.data_pages[1].num_values(), 1);
//     assert_eq!(
//         data_pages.data_pages[1].encoded_values.as_ref(),
//         3i64.as_bytes()
//     );
//
//     Ok(())
// }
