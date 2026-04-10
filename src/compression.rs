use anyhow::Result;
use bytes::Bytes;

use crate::format::CompressionCodec;

/// Decompress bytes into a new allocated [`Bytes`].
///
/// **SNAPPY**: https://docs.rs/snap/latest/snap/raw/struct.Decoder.html#method.decompress_vec
///
/// **GZIP**: The [gzip spec](https://parquet.apache.org/docs/file-format/data-pages/compression/#gzip)
/// mentions we should use multiple GZIP members: https://docs.rs/flate2/latest/flate2/read/struct.MultiGzDecoder.html
///
/// **BROTLI**: https://docs.rs/brotli/latest/brotli/struct.Decompressor.html
///
/// **ZSTD**: https://docs.rs/zstd/latest/zstd/stream/read/struct.Decoder.html
///
/// **LZ4_RAW**: https://docs.rs/lz4_flex/latest/lz4_flex/block/fn.decompress.html
///
#[allow(unused_variables)]
pub fn decompress(
    compressed_data: Bytes,
    codec: CompressionCodec,
    uncompressed_size: usize,
) -> Result<Bytes> {
    match codec {
        CompressionCodec::UNCOMPRESSED => Ok(compressed_data),
        CompressionCodec::SNAPPY => todo!(),
        CompressionCodec::GZIP => todo!(),
        CompressionCodec::BROTLI => todo!(),
        CompressionCodec::ZSTD => todo!(),
        CompressionCodec::LZ4_RAW => todo!(),
        _ => unimplemented!("Unsupported codec: {}", codec.0),
    }
}
