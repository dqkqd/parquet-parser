pub mod bit_packed;
pub mod plain;
pub mod rle;
pub mod rle_bit_packing_hybrid;
pub mod uleb128;

use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    format::{Encoding, Type},
    page::Page,
};

/// Decode a data page into a vector of [`Scalar`] using correct decoder.
#[allow(unused_variables)]
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
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
