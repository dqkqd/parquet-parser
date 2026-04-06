use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

#[allow(unused_variables)]
pub fn rle_boolean_decode(encoded_data: &mut Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    todo!()
}
