use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

#[allow(unused_variables)]
pub fn dictionary_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
