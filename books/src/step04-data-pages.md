# Data Pages

As noted in the [Understand file structure](./step02-file-structure.md), a column chunk has multiple
pages, all packed together.

![all pages in a column chunks are written back to back](images/pages-in-a-column-chunk-are-written-back-to-back.png)

The `ColumnMetaData` from the
[metadata spec](https://parquet.apache.org/docs/file-format/metadata/#file-metadata) contains
everything we need to extract a column chunk data out.

- `data_page_offset`: the offset of a column chunk in a parquet file
- `total_compressed_size`: the length of a column chunk data, this includes multiple pages packed
  together.

![column metadata stores position and length of the column data](images/column-metadata-stores-position-and-length.png)

All pages for a column chunk is represented in code as a `Pages` struct. At the moment, we only
focus on `data_pages`.

```rust,ignore
pub struct Pages {
    pub data_pages: Vec<Page>,
    pub dictionary_page: Option<Page>,
}
```

## Task

Implement the `read_pages` function in `src/page.rs`, it takes the whole file in `Bytes` and returns
a `Pages`. You should use `read_page` function from the [previous step](./step03-data-page.md) and
keep parsing until there is no page left.

```rust,ignore
pub fn read_pages(data: Bytes, column_metadata: &ColumnMetaData) -> Result<Pages> {
    todo!("step04: read all pages for a given column chunk")
}
```

## Test

Test case for this step is `step04_data_pages`.

## Hints and Solution

<details>
  <summary>Hint (how to get all column chunk data)</summary>

The column chunk data position and its length are stored in `data_page_offset` and
`total_compressed_size`.

```rust,ignore
let column_chunk_data = data.slice(data_page_offset..data_page_offset + total_compressed_size)
```

</details>

<details>
  <summary>Hint (how to extract all pages)</summary>

The `read_page` function returns the remaining bytes. Keep applying `read_page` until there is no
bytes left.

```rust,ignore
while !data.is_empty() {
    let (page, remaining) = read_page(/* ... */);
    data = remaining;
}
```

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn read_pages(data: Bytes, column_metadata: &ColumnMetaData) -> Result<Pages> {
    let offset = column_metadata.data_page_offset as usize;
    let len = column_metadata.total_compressed_size as usize;

    let mut pages_bytes = data.slice(offset..offset + len);
    let mut data_pages = vec![];

    while !pages_bytes.is_empty() {
        let (page, remaining) = read_page(pages_bytes, column_metadata.codec)?;
        data_pages.push(page);
        pages_bytes = remaining;
    }

    Ok(Pages {
        data_pages,
        dictionary_page: None,
    })
}
```

</details>
