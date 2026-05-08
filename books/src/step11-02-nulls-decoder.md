# Nulls Decoder

In this step, we will add missing entries to decoded columns. One thing to note is that the data
page doesn't include the missing entries in its encoded data (even though `num_values` still refers
to the total values in a page).

![nulls in data pages and definition levels, including num_values](images/nulls-data-pages-with-num-values.png)

## Task

### `add_nulls_entries`

Implement the `add_nulls_entries` function in `src/nulls.rs`. It takes a null map, a decoded vector
of `Scalar`, and returns a new vector of `Scalar` containing null entries.

```rust,ignore
pub fn add_nulls_entries(
    is_present: &[bool],
    scalars: Vec<Scalar>,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    todo!("step11-02: handle nulls in a column")
}
```

To create a null `Scalar`, you might find the `scalar_null` helper useful.

### `read_column`

Update the `read_column` function to handle null values. You should compute the null map and add
missing entries to the decoded column.

```rust,ignore
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    // ...
}
```

The `decode_page` function needs the actual number of values encoded in a page to decode the data.
However the `num_values` in the page header is the total number of values including nulls. You must
handle them correctly when decoding a page.

## Test

Test case for this step is `step11_02_nulls_decoder`.

## Hints and Solution

<details>
  <summary>Solution</summary>

`add_nulls_entries`:

```rust,ignore
pub fn add_nulls_entries(
    is_present: &[bool],
    scalars: Vec<Scalar>,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    let mut scalars = scalars;
    scalars.reverse();

    let mut result = Vec::with_capacity(is_present.len());
    for present in is_present {
        if *present {
            result.push(scalars.pop().with_context(
                || "add_nulls_entries: scalars is empty! the nulls map isn't correct",
            )?);
        } else {
            result.push(Scalar::null(parquet_to_polars_type(parquet_type)))
        }
    }

    Ok(result)
}
```

`read_column`:

```rust,ignore
pub fn read_column(data: Bytes, column_chunk: &ColumnChunk) -> Result<Column> {
    // ...
    for page in pages.data_pages {
        // compute the null map from the definition levels
        let is_present = decode_definition_levels(&page)?;
        // compute the actual number of values encoded in a page
        let num_values = is_present.iter().filter(|v| **v).count();

        let decoded_scalars = decode_page(&page, column_metadata.type_, num_values)?;
        let decoded_scalars =
            add_nulls_entries(&is_present, decoded_scalars, column_metadata.type_)?;

        scalars.extend(decoded_scalars);
    }
    // ...
}
```

</details>
