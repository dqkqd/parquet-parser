use anyhow::Result;
use bytes::Bytes;

use crate::format::FileMetaData;

/// Read a parquet file's [`FileMetaData`].
///
/// ```text
/// ...
/// File Metadata
/// 4-byte length in bytes of file metadata (little endian)
/// 4-byte magic number "PAR1"
/// ```
///
/// [file-format]: https://parquet.apache.org/docs/file-format/
#[allow(unused_variables)]
pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    // Use `crate::thrift::read_thrift_metadata` to read thrift encoded data
    todo!()
}
