use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// Plain decoding
///
/// - BOOLEAN: Bit Packed, LSB first
/// - INT32: 4 bytes little endian
/// - INT64: 8 bytes little endian
/// - FLOAT: 4 bytes IEEE little endian
/// - DOUBLE: 8 bytes IEEE little endian
/// - BYTE_ARRAY: length in 4 bytes little endian followed by the bytes contained in the array
/// - FIXED_LEN_BYTE_ARRAY: the bytes contained in the array
///
/// https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN
#[allow(unused)]
pub fn plain_decode(
    encoded_data: &mut Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    match parquet_type {
        Type::BOOLEAN => todo!("Handle bit packed later"),
        Type::INT32 => todo!(),
        Type::INT64 => todo!(),
        Type::FLOAT => todo!(),
        Type::DOUBLE => todo!(),
        Type::BYTE_ARRAY => todo!(),
        Type::FIXED_LEN_BYTE_ARRAY => todo!(),
        _ => unimplemented!("Unsupported type {} for PLAIN encoding", parquet_type.0),
    }
}
