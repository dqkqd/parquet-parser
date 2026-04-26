use anyhow::Result;
use bytes::Bytes;

use crate::format::{ColumnMetaData, CompressionCodec, Encoding, PageHeader};

/// TODO: docs
#[derive(Debug)]
pub struct DataPage {
    pub page_header: PageHeader,
    pub definition_levels: Option<Bytes>,
    pub encoded_values: Bytes,
}

impl DataPage {
    // TODO: docs
    pub fn num_values(&self) -> i32 {
        todo!()
    }

    // TODO: docs
    pub fn encoding(&self) -> Encoding {
        todo!()
    }
}

/// Read the custom [`DataPage`]. TODO: diagram
///
/// ```text
/// ┌────────┬───────────────────────┬──────────────────┬──────────────────┬────────────────┐
/// │  PAR1  │       PageHeader      │ repetition_level │ definition_level │ encoded_values │
/// │        │                       │                  │                  │                │
/// │ 4-byte │ total_compressed_size │ 4-byte + RLE run │ 4-byte + RLE run │                │
/// └────────┼───────────────────────┴──────────────────┴──────────────────┴────────────────┘
///          │                       └──────────────────────────────────────────────────────┘
///   data_page_offset                                  compressed_page_size
/// ```
#[allow(unused_variables)]
pub fn read_data_page(data: Bytes, codec: CompressionCodec) -> Result<(DataPage, Bytes)> {
    todo!()
}

#[derive(Debug)]
pub struct ColumnDataPages {
    pub data_pages: Vec<DataPage>,
    pub dictionary_page: Option<DataPage>,
}

/// TODO: docs
#[allow(unused_variables)] // TODO: remove this
pub fn read_column_data_pages(
    data: Bytes,
    column_metadata: &ColumnMetaData,
) -> Result<ColumnDataPages> {
    todo!()
}
