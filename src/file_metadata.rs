use anyhow::Result;
use bytes::{Buf, Bytes};

use crate::{format::FileMetaData, thrift::read_thrift_metadata};

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
pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    let metadata_size = data.slice(data.len() - 8..).get_u32_le() as usize;
    let metadata_bytes = data.slice(data.len() - 8 - metadata_size..data.len() - 8);
    let (metadata, _) = read_thrift_metadata::<FileMetaData>(metadata_bytes)?;
    Ok(metadata)
}
