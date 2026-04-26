mod integration;

use std::collections::HashMap;

use anyhow::Result;
use bytes::Bytes;
use parquet::basic::{Compression, Encoding};
use parquet_parser::{format::Type, writer::write_parquet};

pub fn make_parquet(
    data: &str,
    dictionary_enabled: bool,
    encoding: Encoding,
    compression: Compression,
    rows_per_page: Option<usize>,
    rows_per_group: Option<usize>,
    data_types_override: Option<HashMap<&str, Type>>,
) -> Result<Bytes> {
    let input = data.trim().as_bytes().to_vec();
    let data_types_override = data_types_override.map(|dtypes| {
        let dtypes: HashMap<String, Type> = dtypes
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
        dtypes
    });
    let out = write_parquet(
        input,
        dictionary_enabled,
        encoding,
        compression,
        rows_per_page,
        rows_per_group,
        data_types_override,
    )?;
    Ok(Bytes::from(out))
}
