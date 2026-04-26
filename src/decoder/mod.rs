pub mod bit_packed;
pub mod plain;
pub mod rle;
pub mod rle_bit_packing_hybrid;
pub mod uleb128;

use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    data_page::DataPage,
    format::{Encoding, Type},
};

/// Decode a data page into a vector of [`Scalar`] using correct decoder.
#[allow(unused_variables)]
pub fn decode_data_page(
    data_page: DataPage,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    match data_page.encoding() {
        Encoding::PLAIN => todo!(),
        Encoding::RLE => todo!(),
        Encoding::RLE_DICTIONARY => todo!(),
        e => unimplemented!("decode_data_page: unsupported encoding {:?}", e),
    }
}

/// Decode a definition levels for nulls handling.
#[allow(unused_variables)]
pub fn decode_definition_levels(
    definition_levels: Bytes,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
