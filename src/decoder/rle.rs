use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// TODO: docs
#[allow(unused_variables)]
pub fn rle_decode(
    encoded_data: &mut Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
