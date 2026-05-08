# Column

We know how to get all pages for a column chunk and how to decode an individual page. Now, let's put
all of them together and completely parse a column chunk.

![a column chunk contains multiple pages](images/a-column-chunk-contains-multiple-pages.png)

To represent a column, we use
[Polars Column](https://docs.rs/polars/latest/polars/prelude/enum.Column.html).

## Task

Implement the `read_column` function in `src/column.rs`. It takes the entire file data as `Bytes`
and returns a `Column` data for a given column chunk.

```rust,ignore
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    todo!("step06: implement read column")
}
```

Some important notes:

- Everything you need to parse column data is stored in the column metadata
- To get the number of values in a page, you can use `Page::num_values()`
- To convert a vector of `Scalar` to `Column`, you might find the helper `column_from_scalars`
  useful

## Test

Test case for this step is `step06_column`.

## Hints and Solution

<details>
  <summary>Hint (how to get the column metadata)</summary>

The column metadata is stored as `meta_data` field in a `ColumnChunk`.

```rust,ignore
column_chunk
    .meta_data
    .as_ref()
    .expect("read_column: missing column metadata");
```

</details>

<details>
  <summary>Hint (how to get the parquet data type)</summary>

The parquet data type can be retrieved from `column_metadata.type_`.

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    let column_metadata = column_chunk
        .meta_data
        .as_ref()
        .expect("read_column: missing column metadata");
    let pages = read_pages(data, column_metadata)?;
    let mut scalars = Vec::with_capacity(column_metadata.num_values as usize);
    for page in pages.data_pages {
        let decoded_scalars = decode_page(&page, column_metadata.type_, page.num_values())?;
        scalars.extend(decoded_scalars);
    }
    column_from_scalars(scalars, column_metadata)
}
```

</details>
