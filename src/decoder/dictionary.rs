use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

#[allow(unused)] // avoid using different `Type` when implementing solution
use crate::format::Type;

#[allow(unused_variables)]
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    todo!()
}
