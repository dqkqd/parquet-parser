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
    let values: Vec<AnyValue<'_>> = scalars
        .into_iter()
        .map(|scalar| scalar.into_value())
        .collect();
    let series = Series::from_any_values(column_name.into(), &values, true)?;
    let column = Column::from(series);
    Ok(column)
}

/// Read a [`Column`] from a parquet data.
/// TODO: docs
#[allow(unused_variables)]
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    let column_metadata = column_chunk
        .meta_data
        .as_ref()
        .expect("read_column: missing column metadata");
    // You should:
    // - Get all the pages using `read_column_data_pages`
    // - Decode all the pages using appropriate decoder into `scalars`
    column_from_scalars(vec![], &column_name(column_metadata))
}
