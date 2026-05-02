use anyhow::Result;
use bytes::Bytes;

use crate::format::{ColumnMetaData, CompressionCodec, Encoding, PageHeader};

pub enum Page {
    /// A data page contains all the data for a specific columns.
    /// At the moment, the parser cannot handle nested data type, so `repetition_levels` must not be presented.
    ///
    /// ```text
    /// ┌───────────────────────┬───────────────────┬───────────────────┬────────────────┐
    /// │       PageHeader      │ repetition_levels │ definition_levels │ encoded_values │
    /// │                       │                   │                   │                │
    /// │                       │     <ignore>      │ 4-byte + RLE run  │                │
    /// └───────────────────────┴───────────────────┴───────────────────┴────────────────┘
    ///                         └────────────────────────────────────────────────────────┘
    ///                                            compressed_page_size
    /// ```
    ///
    /// [data page file format]: https://parquet.apache.org/docs/file-format/data-pages/
    DataPage {
        page_header: PageHeader,
        definition_levels: Bytes,
        encoded_values: Bytes,
    },
    /// A dictionary page contains the actual values for a column chunk, the actual indexes are store
    /// in the data page itself.
    ///
    /// ```text
    /// ┌───────────────────────┬────────────────┐
    /// │       PageHeader      │ encoded_values │
    /// │                       │                │
    /// │                       │                │
    /// └───────────────────────┴────────────────┘
    ///                         └────────────────┘
    ///                        compressed_page_size
    /// ```
    ///
    /// [dictionary page for column chunk]: https://parquet.apache.org/docs/file-format/data-pages/columnchunks/
    /// [dictionary encoding]: https://parquet.apache.org/docs/file-format/data-pages/encodings/#DICTIONARY
    DictionaryPage {
        page_header: PageHeader,
        encoded_values: Bytes,
    },
}

impl Page {
    pub fn num_values(&self) -> usize {
        match self {
            Page::DataPage { page_header, .. } => {
                page_header.data_page_header.as_ref().unwrap().num_values as usize
            }
            Page::DictionaryPage { page_header, .. } => {
                page_header
                    .dictionary_page_header
                    .as_ref()
                    .unwrap()
                    .num_values as usize
            }
        }
    }

    pub fn encoding(&self) -> Encoding {
        match self {
            Page::DataPage { page_header, .. } => {
                page_header.data_page_header.as_ref().unwrap().encoding
            }
            Page::DictionaryPage { page_header, .. } => {
                page_header
                    .dictionary_page_header
                    .as_ref()
                    .unwrap()
                    .encoding
            }
        }
    }

    pub fn encoded_values(&self) -> Bytes {
        match self {
            Page::DataPage { encoded_values, .. } => encoded_values.clone(),
            Page::DictionaryPage { encoded_values, .. } => encoded_values.clone(),
        }
    }
}

/// Read [`Page`].
///
/// This function receive a `data` at page boundary, and `codec` (to handle compression),
/// returning the [`Page`] and remaining data in [`Bytes`].
///
/// Look at [`Page`] enum definition for bytes level representation.
#[allow(unused_variables)]
pub fn read_page(data: Bytes, codec: CompressionCodec) -> Result<(Page, Bytes)> {
    todo!()
}

/// TODO: docs
/// Read all needed pages for a single column chunk.
/// Each page is written back to back, we should keep parsing each of them until there is no remaining bytes left.
/// TODO: ascii
#[allow(unused_variables)]
pub fn read_pages(data: Bytes, column_metadata: &ColumnMetaData) -> Result<Vec<Page>> {
    todo!()
}
