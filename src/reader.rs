use std::path::Path;
use std::{fs::File, io::Read};

use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::{
    file_metadata::read_file_metadata, magic::ensure_header_footer_magic,
    row_group::read_row_groups,
};

/// Read a parquet file into [`DataFrame`].
///
/// This function verifies if the magic number is correct,
/// reads the file metadata, then parses all row groups into the [`DataFrame`].
pub fn read_parquet(file_path: impl AsRef<Path>) -> Result<DataFrame> {
    let mut file = File::open(file_path)?;
    let mut buf = Vec::new();
    file.read_to_end(&mut buf)?;

    let data = Bytes::from(buf);
    ensure_header_footer_magic(data.clone())?;
    let file_metadata = read_file_metadata(data.clone())?;
    let df = read_row_groups(data, &file_metadata)?;
    Ok(df)
}
