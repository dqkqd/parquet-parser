use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::{ColumnChunk, ColumnMetaData};

/// Get column name from [`ColumnMetaData::path_in_schema`].
/// Since our parser doesn't handle nested data type, `path_in_schema` always contains has length 1.
#[allow(unused)]
fn column_name(column_metadata: &ColumnMetaData) -> String {
    column_metadata.path_in_schema.join(".")
}

/// Convert a vector of [`Scalar`] to [`Column`].
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

/// Read [`Column`] from a parquet file based on [`ColumnChunk`]'s metadata.
///
/// A column chunk contains multiple pages, this functions need to get all of them
/// and decode each page individually.
///
/// TODO: diagram contains a column chunk with multiple columns
#[allow(unused_variables)]
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    let column_metadata = column_chunk
        .meta_data
        .as_ref()
        .expect("read_column: missing column metadata");
    // You should:
    // - Get all the pages using `read_column_data_pages`
    // - Decode all the pages into `scalars`
    column_from_scalars(vec![], &column_name(column_metadata))
}

/// TODO: docs
#[allow(unused)]
fn dictionary_lookup(
    dictionary_entries: &Option<Vec<Scalar>>,
    indices_or_scalars: Vec<Scalar>,
) -> Result<Vec<Scalar>> {
    todo!()
}
