use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// RLE and bit packed decode.
/// [Full spec can be found here](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE)
///
/// ```text
/// rle-bit-packed-hybrid: <length> <encoded-data>
/// // length is not always prepended, please check the table below for more detail
/// length := length of the <encoded-data> in bytes stored as 4 bytes little endian (unsigned int32)
/// encoded-data := <run>*
/// run := <bit-packed-run> | <rle-run>
/// ...
/// bit-packed-run := <bit-packed-header> <bit-packed-values>
/// bit-packed-header := varint-encode(<bit-pack-scaled-run-len> << 1 | 1)
/// ...
/// rle-run := <rle-header> <repeated-value>
/// rle-header := varint-encode( (rle-run-len) << 1)
/// ```
///
/// Both rle and bit packed header can be decoded using [ULEB128](https://en.wikipedia.org/wiki/LEB128):
///  `header := varint-encode(...)`.
///
/// The bit packed header has `LSB = 1`, because its `run-len` is OR-ed with `1` before encoding.
/// `<bit-pack-scaled-run-len> << 1 | 1`.
/// And the rle header has `LSB = 0`.
///
/// ## RLE run decode
///
/// ```text
/// rle-run := <rle-header> <repeated-value>
/// rle-header := varint-encode( (rle-run-len) << 1)
/// rle-run-len := *see 3 below*
/// repeated-value := value that is repeated, using a fixed-width of round-up-to-next-byte(bit-width)
/// ```
///
/// ## Bit packed decode
///
/// ```text
/// bit-packed-run := <bit-packed-header> <bit-packed-values>
/// bit-packed-header := varint-encode(<bit-pack-scaled-run-len> << 1 | 1)
/// // we always bit-pack a multiple of 8 values at a time, so we only store the number of values / 8
/// bit-pack-scaled-run-len := (bit-packed-run-len) / 8
/// bit-packed-run-len := *see 3 below*
/// bit-packed-values := *see 1 below*
///
/// TODO: RLE runs when they have more than 8 repeated values
/// java references: https://github.com/apache/parquet-java/blob/4c8f4d4b875259e2ece5f96c5ee90a03f78805ec/parquet-column/src/main/java/org/apache/parquet/column/values/rle/RunLengthBitPackingHybridEncoder.java#L163
/// rust parquet references: https://github.com/apache/arrow-rs/blob/68851ef953fd771cc310203c446e54145d4407e1/parquet/src/encodings/rle.rs#L140
/// ```
#[allow(unused_variables)]
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: &mut Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}

// TODO: docs
#[allow(unused_variables)]
pub fn rle_bit_packing_hybrid_run_decode(
    encoded_data: &mut Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!()
}
