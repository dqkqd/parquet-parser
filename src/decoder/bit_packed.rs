use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// Decode bit packed encoding data.
///
/// TODO: diagram
/// [parquet encoding spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE)
/// TODO: docs. Only support boolean and u32
#[allow(unused)]
pub fn bit_packed_decode(
    mut encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
