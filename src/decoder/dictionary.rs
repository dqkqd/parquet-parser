use anyhow::Result;
use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::{decoder::rle_bit_packing_hybrid::rle_bit_packing_hybrid_decode, format::Type};

/// Dictionary decoding for data page.
///
/// The data page in dictionary encoding use RLE Bit-packing hybrid encoding.
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let bit_width = encoded_data.get_u8();
    rle_bit_packing_hybrid_decode(encoded_data, Type::INT32, bit_width, num_values, false)
}
