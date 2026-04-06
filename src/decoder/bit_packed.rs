use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// TODO: docs. Only support boolean and u32
#[allow(unused_variables)]
pub fn bit_packed_decode(
    encoded_data: &mut Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
