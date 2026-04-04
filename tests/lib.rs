#[cfg(false)]
mod magic;

#[cfg(false)]
mod file_metadata;

#[cfg(false)]
mod data_page;

use std::{io::Cursor, sync::Arc};

use anyhow::Result;
use arrow::csv::{ReaderBuilder, reader::Format};
use bytes::Bytes;
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, Encoding},
    file::properties::{DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT, WriterProperties},
};

pub fn make_parquet(
    data: &str,
    dictionary_enabled: bool,
    encoding: Encoding,
    compression: Compression,
    rows_per_page: Option<usize>,
    rows_per_group: Option<usize>,
) -> Result<Bytes> {
    let props = WriterProperties::builder()
        .set_encoding(encoding)
        .set_compression(compression)
        .set_dictionary_enabled(dictionary_enabled)
        .set_created_by("Hello parquet!".to_string())
        .set_data_page_row_count_limit(rows_per_page.unwrap_or(DEFAULT_DATA_PAGE_ROW_COUNT_LIMIT))
        .set_write_batch_size(1) // ensure we don't write to page boundary
        .set_max_row_group_row_count(rows_per_group)
        .build();

    let mut cursor = Cursor::new(data.trim().as_bytes());
    let format = Format::default().with_header(true);
    let (schema, _) = format.infer_schema(&mut cursor, None)?;
    let schema = Arc::new(schema);

    cursor.set_position(0);
    let reader = ReaderBuilder::new(Arc::clone(&schema))
        .with_header(true)
        .with_delimiter(b',')
        .build(cursor)?;

    let mut out = Vec::new();
    {
        let mut writer = ArrowWriter::try_new(&mut out, Arc::clone(&schema), Some(props))?;
        for batch in reader {
            let batch = batch?;
            writer.write(&batch)?;
        }
        writer.close()?;
    }

    Ok(Bytes::from(out))
}
