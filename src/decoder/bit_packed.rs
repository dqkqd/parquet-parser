use anyhow::Result;
use bytes::{Buf, Bytes};
use polars::prelude::*;

use crate::format::Type;

/// Decode bit-packed encoded data.
///
/// This function bit-packed encoded data into a vector of [`Scalar`].
/// It only supports 2 type of data type: BOOLEAN and INT32.
///
/// [parquet encoding spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE)
pub fn bit_packed_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let mut scalars = Vec::with_capacity(num_values);

    let mask = u64::MAX >> (64 - bit_width);
    let mut buffer = 0;
    let mut buffer_bits = 0;

    while scalars.len() < num_values {
        // Buffer needs more bits
        while buffer_bits < bit_width {
            let group = encoded_data.get_u8() as u64;
            // put the group data to the left of the current buffer
            buffer |= group << buffer_bits;
            buffer_bits += 8;
        }

        let scalar = match parquet_type {
            Type::BOOLEAN => Scalar::from(buffer & 1 == 1),
            Type::INT32 => Scalar::from((buffer & mask) as i32),
            _ => unimplemented!("bit_packed_decode: unsupported type: {:?}", parquet_type),
        };
        scalars.push(scalar);

        buffer = buffer.checked_shr(bit_width as u32).unwrap_or(0);
        buffer_bits -= bit_width;
    }

    Ok(scalars)
}
