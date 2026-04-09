use anyhow::Result;
use bytes::Bytes;
use polars::prelude::*;

use crate::format::RowGroup;

/// Read a [`DataFrame`] from [`RowGroup`]
#[allow(unused_variables)]
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    todo!()
}

/// Read a [`DataFrame`] from a vector of [`RowGroup`].
#[allow(unused_variables)]
pub fn read_row_groups(data: Bytes, row_groups: &[RowGroup]) -> Result<DataFrame> {
    todo!()
}
