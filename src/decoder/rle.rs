use anyhow::{Context, Result};
use bytes::Bytes;
use polars::prelude::*;

use crate::{decoder::bit_packed::bit_packed_decode, format::Type};

/// RLE decoding.
///
/// A RLE encoded data includes the number of repeated values and the actual values.
/// This function takes the repeated value as an encoded data and
/// returns a vector contains the repeated values.
///
/// *Yes, this is very inefficient!*
pub fn rle_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let scalar = bit_packed_decode(encoded_data, parquet_type, bit_width, 1)?
        .pop()
        .with_context(|| "rle_decode: cannot get decoded scalar from `bit_packed_decode`")?;
    let scalars = vec![scalar; num_values];
    Ok(scalars)
}
