use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    column::read_column,
    format::{FileMetaData, RowGroup},
};

/// Read a row group into [`DataFrame`].
///
/// A row group contains multiple column chunks.
/// This function reads all the column chunks into a single [`DataFrame`].
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    let mut columns = Vec::with_capacity(row_group.columns.len());
    for column_chunk in &row_group.columns {
        let column = read_column(data.clone(), column_chunk)?;
        columns.push(column);
    }
    let df = DataFrame::new_infer_height(columns)?;
    Ok(df)
}

/// Read row groups into [`DataFrame`].
///
/// A file contains multiple row groups.
/// This function reads all the row groups, and
/// concatenate all the returned [`DataFrame`]s into a single [`DataFrame`].
pub fn read_row_groups(data: Bytes, file_metadata: &FileMetaData) -> Result<DataFrame> {
    let mut dfs = Vec::with_capacity(file_metadata.row_groups.len());
    for row_group in &file_metadata.row_groups {
        let df = read_row_group(data.clone(), row_group)?;
        dfs.push(df.lazy());
    }
    let df = concat(
        dfs,
        UnionArgs {
            strict: true,
            ..Default::default()
        },
    )?
    .collect()?;
    Ok(df)
}
