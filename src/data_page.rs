use anyhow::Result;
use bytes::Bytes;

use crate::format::{ColumnMetaData, CompressionCodec, Encoding, PageHeader, PageType};

/// A struct represents [DataPage](https://parquet.apache.org/docs/file-format/data-pages/).
///
/// At the moment, the parser cannot handle nested data type, so `repetition_levels` must not be presented.
/// Depends on the page type, `definition_levels` might not be exists.
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
#[derive(Debug)]
pub struct DataPage {
    pub page_header: PageHeader,
    pub definition_levels: Option<Bytes>,
    pub encoded_values: Bytes,
}

impl DataPage {
    /// Return the number of number of rows stored in this page.
    pub fn num_values(&self) -> usize {
        match self.page_header.type_ {
            PageType::DATA_PAGE => todo!(),
            PageType::DICTIONARY_PAGE => todo!(),
            page_type => unimplemented!("DataPage::num_values, unsupported page {:?}", page_type),
        }
    }

    /// Return the encoding type used in this page.
    pub fn encoding(&self) -> Encoding {
        match self.page_header.type_ {
            PageType::DATA_PAGE => todo!(),
            PageType::DICTIONARY_PAGE => todo!(),
            page_type => unimplemented!("DataPage::encoding, unsupported page {:?}", page_type),
        }
    }
}

/// Read [`DataPage`].
///
/// This function receive a `data` at page boundary, and `codec` (to handle compression),
/// returning the [`DataPage`] and remaining data in [`Bytes`].
/// Look at [`DataPage`] struct definition for bytes level representation.
#[allow(unused_variables)]
pub fn read_data_page(data: Bytes, codec: CompressionCodec) -> Result<(DataPage, Bytes)> {
    todo!()
}

/// All necessary pages for a single column chunk.
#[derive(Debug)]
pub struct ColumnDataPages {
    pub data_pages: Vec<DataPage>,
    pub dictionary_page: Option<DataPage>,
}

/// Read all needed pages for a single column chunk.
/// Each page is packed together, we should keep parsing each of them until there is no remaining data.
/// TODO: ascii
#[allow(unused_variables)] // TODO: remove this
pub fn read_column_data_pages(
    data: Bytes,
    column_metadata: &ColumnMetaData,
) -> Result<ColumnDataPages> {
    todo!()
}
