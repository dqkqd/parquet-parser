use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

/// Dictionary decoding for data page.
///
/// The data page in dictionary encoding use RLE Bit-packing hybrid encoding.
#[allow(unused_variables)]
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    todo!("step12: implement dictionary decoder")
}

/// Map the values indexes from data page to the correct values using dictionary entries.
///
/// - `dictionary_entries`: a vector of dictionary values from the dictionary page.
/// - `indexes_or_scalars`: This is either indexes from the data page.
#[allow(unused_variables)]
pub fn map_dictionary_entries(
    dictionary_entries: &[Scalar],
    indexes: Vec<Scalar>,
) -> Result<Vec<Scalar>> {
    todo!("step12-01: implement dictionary decoder for two values")
}
