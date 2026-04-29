use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// RLE decoding.
///
/// A RLE encoded data includes the number of repeated values and the actual values.
///
/// This functions takes an encoded data and return a vector contains of the repeated values.
///
/// *This is of courses, very inefficient!*
#[allow(unused)]
pub fn rle_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
