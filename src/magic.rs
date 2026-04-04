use anyhow::Result;
use bytes::Bytes;

/// Ensure the header and footer contain magic number `PAR1`.
///
/// ```text
/// 4-byte magic number "PAR1"
/// ...
/// File Metadata
/// 4-byte length in bytes of file metadata (little endian)
/// 4-byte magic number "PAR1"
/// ```
///
/// [file-format]: https://parquet.apache.org/docs/file-format/
#[allow(unused_variables)]
pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    todo!()
}
