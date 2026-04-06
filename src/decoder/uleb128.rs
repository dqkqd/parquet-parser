use anyhow::Result;
use bytes::Bytes;

/// A LEB128 decoder: https://en.wikipedia.org/wiki/LEB128
///
/// This is the actual example from wikipedia: encode `624485` and then decode it back
///
/// ```text
/// MSB ---------------------- LSB
///       10011000011101100101  In raw binary
///      010011000011101100101  Padded to a multiple of 7 bits
///  0100110  0001110  1100101  Split into 7-bit groups
/// 00100110 10001110 11100101  Add high 1 bits on all but last (most significant) group to form bytes
///     0x26     0x8E     0xE5  In hexadecimal
///
/// → 0xE5 0x8E 0x26            Output stream (LSB to MSB)
/// ```
///
/// We can decode it like this:
///
/// ```text
/// 0xE5 0x8E 0x26              Encoded data
/// 11100101 10001110 00100110  Binary representation
/// 11100101 10001110 00100110  Get the data until we see group with `LSB = 0`. **This is the important part!**
///  1100101  0001110  0100110  Remove the LSB
///  0100110  0001110  1100101  Reverse it and compute the actual value
/// ```
#[allow(unused_variables)]
pub fn uleb128_decode(encoded_data: &mut Bytes) -> Result<u64> {
    todo!()
}
