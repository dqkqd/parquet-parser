use anyhow::Result;
use bytes::Bytes;

use crate::format::CompressionCodec;

/// Decompress bytes into a new allocated [`Bytes`].
///
/// **SNAPPY**: https://docs.rs/snap/latest/snap/raw/struct.Decoder.html#method.decompress_vec
pub fn decompress(compressed_data: Bytes, codec: CompressionCodec) -> Result<Bytes> {
    match codec {
        CompressionCodec::UNCOMPRESSED => Ok(compressed_data),
        CompressionCodec::SNAPPY => {
            let mut decompressor = snap::raw::Decoder::new();
            let buf = decompressor.decompress_vec(compressed_data.as_ref())?;
            Ok(Bytes::from(buf))
        }
        _ => unimplemented!("Unsupported codec: {}", codec.0),
    }
}
