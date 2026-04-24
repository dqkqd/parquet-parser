use anyhow::Result;
use bytes::Bytes;

/// Ensure we are reading a parquet file by checking the header and footer containing magic number `PAR1`
///
/// ```text
/// 4-byte magic number "PAR1"
/// <data>
/// 4-byte magic number "PAR1"
/// ```
///
/// [file-format]: https://parquet.apache.org/docs/file-format/
#[allow(unused_variables)]
pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    todo!()
}
