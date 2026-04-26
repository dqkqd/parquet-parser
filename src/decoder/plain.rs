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
///
/// This function decode the data into a vector of [`Scalar`].
///
/// [plain encoding]: https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN
/// TODO: do not return remaining.
#[allow(unused)]
pub fn plain_decode(
    mut encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    match parquet_type {
        Type::INT32 => todo!(),
        Type::INT64 => todo!(),
        Type::FLOAT => todo!(),
        Type::DOUBLE => todo!(),
        Type::BYTE_ARRAY => todo!(),
        Type::BOOLEAN => todo!("Plain decoder: unsupported boolean for now, implement later"),
        _ => unimplemented!("Plain decoder: unsupported data type {:?}", parquet_type),
    }
}
