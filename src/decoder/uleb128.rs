use anyhow::{Result, bail};
use bytes::{Buf, Bytes};

/// A ULEB123 decoder: https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128
pub fn uleb128_decode(encoded_data: Bytes) -> Result<(u64, Bytes)> {
    let mut encoded_data = encoded_data;
    let mut result = 0u64;

    let total_bytes = encoded_data.len();
    for i in 0..total_bytes {
        let byte = encoded_data.get_u8() as u64;
        result |= (byte & 0x7F) << (i * 7);
        // MSB = 0, stop
        if byte & 0x80 == 0 {
            return Ok((result, encoded_data));
        }
    }
    bail!("uleb128_decode: no byte with leading 0")
}
