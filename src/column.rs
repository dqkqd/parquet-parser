use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::{ColumnChunk, ColumnMetaData};

/// Get column name from [`ColumnMetaData::path_in_schema`].
/// TODO: docs
#[allow(unused)]
fn column_name(column_metadata: &ColumnMetaData) -> String {
    column_metadata.path_in_schema.join(".")
}

/// TODO: docs
#[allow(unused)]
fn column_from_scalars(scalars: Vec<Scalar>, column_name: &str) -> Result<Column> {
    todo!()
}

/// Read a [`Column`] from a parquet data.
/// TODO: docs
#[allow(unused_variables)]
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    todo!()
}
