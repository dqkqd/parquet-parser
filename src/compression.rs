use anyhow::Result;
use bytes::Bytes;

use crate::format::CompressionCodec;

/// Decompress bytes into a new allocated [`Bytes`].
///
/// **SNAPPY**: https://docs.rs/snap/latest/snap/raw/struct.Decoder.html#method.decompress_vec
pub fn decompress(compressed_data: Bytes, codec: CompressionCodec) -> Result<Bytes> {
    match codec {
        CompressionCodec::UNCOMPRESSED => Ok(compressed_data),
        CompressionCodec::SNAPPY => todo!(),
        _ => unimplemented!("Unsupported codec: {}", codec.0),
    }
}
