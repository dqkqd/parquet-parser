use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::Type;

/// Enum represents a rle bit-packed hybrid run data.
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum RleBitPackedRun {
    BitPacked {
        num_values: usize,
        bit_width: u8,
        encoded_values: Bytes,
    },
    Rle {
        num_values: usize,
        bit_width: u8,
        encoded_values: Bytes,
    },
}

/// Get the correct run from the encoded data.
///
/// This function takes an encoded run and return whether it is a rle run or a bit-packed run.
///
/// ```text
/// run := <bit-packed-run> | <rle-run>
/// ```
/// The `encoded_data` this function receives is for many runs, it has to returns the remaining
/// `encoded_data` so that caller can extract data for the next runs.
///
/// Both rle run and bit-packed run are encoded with a header followed by encoded values:
/// ```text
/// bit-packed-run := <bit-packed-header> <bit-packed-values>
/// rle-run := <rle-header> <repeated-value>
/// ```
/// The header can be decoded using [ULEB128](https://en.wikipedia.org/wiki/LEB128). The decoded header
/// tells us whether this run is bit-packed or rle. If the MSB is 1, then it should be bit-packed, otherwise it is rle.
/// ```text
/// bit-packed-header := varint-encode(<bit-pack-scaled-run-len> << 1 | 1)
/// rle-header := varint-encode( (rle-run-len) << 1)
/// ```
///
/// ## Bit-packed run
///
/// The bit-packed run always pack a multiple of 8 values at a time, so the total number of values (the run-len)
/// should be multiple by 8.
///
/// ```text
/// bit-packed-header := varint-encode(<bit-pack-scaled-run-len> << 1 | 1)
/// bit-pack-scaled-run-len := (bit-packed-run-len) / 8
/// ```
///
/// The total bits needed for the encoded data is: `bit-packed-scaled-run-len * bit-width`
///
/// ## Rle run
///
/// The run length for a rle run is saved as is.
///
/// ```text
/// rle-header := varint-encode( (rle-run-len) << 1)
/// ```
///
/// The total bits needed for the encoded data is: `round-up-to-next-byte(bit-width)`
///
#[allow(unused)]
pub fn get_rle_bit_packed_run(
    encoded_data: Bytes,
    bit_width: u8,
) -> Result<(RleBitPackedRun, Bytes)> {
    todo!()
}

/// RLE bit-packed hybrid decoding.
///
/// The encoded data can contains many runs, each run is either bit-packed run or rle run.
///
/// ```text
/// rle-bit-packed-hybrid: <length> <encoded-data>
/// length is not always prepended, please check the table below for more detail
/// length := length of the <encoded-data> in bytes stored as 4 bytes little endian (unsigned int32)
/// encoded-data := <run>*
/// run := <bit-packed-run> | <rle-run>
/// ```
///
/// If there are more than 8 repeated values, then it will be encoded as a rle run, otherwise bit-packed run.
/// We don't need to care this if we just write a parser. But this is a useful information for writing tests.
///
/// The total values for a bit-packed run is less than or equal 504. This is followed by the official java implementation.
/// https://github.com/apache/parquet-java/blob/4c8f4d4b875259e2ece5f96c5ee90a03f78805ec/parquet-column/src/main/java/org/apache/parquet/column/values/rle/RunLengthBitPackingHybridEncoder.java#L101-L113
///
/// ```
#[allow(unused)]
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    todo!()
}

#[allow(unused)]
/// Decode a single rle bit-packed run.
fn rle_bit_packing_hybrid_run_decode(
    run: RleBitPackedRun,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    todo!()
}
