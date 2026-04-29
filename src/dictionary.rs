use anyhow::Result;
use polars::prelude::*;

use crate::{decoder::decode_page, format::Type, page::Pages};

/// Extracting dictionary entries from [`Pages`].
///
/// Return `None` if [`Pages`] doesn't contain dictionary page (no dictionary encoding used).
/// Otherwise, return the decoded vector of [`Scalar`].
pub fn dictionary_entries(pages: &Pages, parquet_type: Type) -> Result<Option<Vec<Scalar>>> {
    let dictionary_entries = match &pages.dictionary_page {
        Some(page) => {
            let dictionary_entries = decode_page(page, parquet_type, page.num_values())?;
            Some(dictionary_entries)
        }
        None => None,
    };
    Ok(dictionary_entries)
}

/// Try to map the decoded data from data page to the actual values using dictionary entries.
///
/// The dictionary entries might not exist if the page doesn't use dictionary encoding.
/// In that case `dictionary_entries` is `None` and `indexes_or_values` is the actual column values.
#[allow(unused_variables)]
pub fn map_dictionary_entries(
    dictionary_entries: &Option<Vec<Scalar>>,
    indexes_or_values: Vec<Scalar>,
) -> Result<Vec<Scalar>> {
    todo!("step12-02: map indexes in data page to the exact values")
}
