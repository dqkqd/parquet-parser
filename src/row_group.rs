use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::{FileMetaData, RowGroup};

/// Read a row group into [`DataFrame`].
///
/// A row group contains multiple column chunks, this function read all the columns and concat them into a single [`DataFrame`].
#[allow(unused_variables)]
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    todo!()
}

/// Read row groups into [`DataFrame`].
///
/// A parquet file can contains multiple row groups, this function read all the row groups and concat them into a single [`DataFrame`].
#[allow(unused_variables)]
pub fn read_row_groups(data: Bytes, file_metadata: &FileMetaData) -> Result<DataFrame> {
    todo!()
}
