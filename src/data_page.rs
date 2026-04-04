use anyhow::Result;
use bytes::Bytes;

use crate::format::{ColumnMetaData, PageHeader};

/// TODO: docs
#[derive(Debug)]
pub struct DataPage {
    pub page_header: PageHeader,
    pub encoded_values: Bytes,
}

impl DataPage {
    // TODO: docs
    pub fn num_values(&self) -> i32 {
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
pub fn read_data_page(data: &mut Bytes, column_metadata: &ColumnMetaData) -> Result<DataPage> {
    todo!()
}

/// TODO: docs
#[allow(unused_variables)]
pub fn read_data_pages(data: Bytes, column_metadata: &ColumnMetaData) -> Result<Vec<DataPage>> {
    todo!()
}
