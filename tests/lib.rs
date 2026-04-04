#[cfg(false)]
mod magic;

use std::{io::Cursor, sync::Arc};

use anyhow::Result;
use arrow::csv::{ReaderBuilder, reader::Format};
use bytes::Bytes;
use parquet::{
    arrow::ArrowWriter,
    basic::{Compression, Encoding},
    file::properties::WriterProperties,
};

pub fn make_parquet(
    data: &str,
    dictionary_enabled: bool,
    encoding: Encoding,
    compression: Compression,
) -> Result<Bytes> {
    let props = WriterProperties::builder()
        .set_encoding(encoding)
        .set_compression(compression)
        .set_dictionary_enabled(dictionary_enabled)
        .set_created_by("Hello parquet!".to_string())
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
