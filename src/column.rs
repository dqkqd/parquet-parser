use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    decoder::decode_page,
    format::{ColumnChunk, ColumnMetaData},
    page::read_pages,
};

/// Convert a vector of [`Scalar`] to a [`Column`].
fn column_from_scalars(scalars: Vec<Scalar>, column_metadata: &ColumnMetaData) -> Result<Column> {
    let values: Vec<AnyValue<'_>> = scalars
        .into_iter()
        .map(|scalar| scalar.into_value())
        .collect();

    let column_name = column_metadata.path_in_schema.join(".");
    let series = Series::from_any_values(column_name.into(), &values, true)?;
    let column = Column::from(series);

    Ok(column)
}

/// Read [`Column`] from a parquet file based on [`ColumnChunk`]'s metadata.
///
/// A column chunk contains multiple pages, this function extract all the pages,
/// decodes them and returns the correct [`Column`] for a chunk.
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    let column_metadata = column_chunk
        .meta_data
        .as_ref()
        .expect("read_column: missing column metadata");
    let pages = read_pages(data, column_metadata)?;
    let mut scalars = Vec::with_capacity(column_metadata.num_values as usize);
    for page in pages.data_pages {
        let decoded_scalars = decode_page(&page, column_metadata.type_, page.num_values())?;
        scalars.extend(decoded_scalars);
    }
    column_from_scalars(scalars, column_metadata)
}
