# Row Group

A row group contains multiple columns. A file contains multiple row groups. After finishing this
step, we can read all the data in a parquet file.

![row groups and column chunks data](images/a-row-group-contains-multiple-columns-a-file-contains-multiple-row-groups.png)

The relationship above looks like this from the
[metadata spec](https://parquet.apache.org/docs/file-format/metadata/#file-metadata).

![Metadata relationship from the spec, a file metadata contains multiple row groups, a row group contains multiple columns](images/row-groups-metadata-relationship.png)

For representing parsed data in the parser, we use
[polars's DataFrame](https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html), it is
recommended to look at their documentation before implementing the task.

## Task

Implement two functions `read_row_group` and `read_row_groups` in `src/row_group.rs`.

### `read_row_group`

This function takes the whole file data in `Bytes` and returns a `DataFrame`. You can use
[`DataFrame::new_infer_height`](https://docs.rs/polars/latest/polars/frame/struct.DataFrame.html#method.new_infer_height)
to group multiple columns together into a single `DataFrame`.

```rust,ignore
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    todo!("step07: implement read row group")
}
```

### `read_row_groups`

This function takes the whole file data in `Bytes` and returns a `DataFrame`. You can use
[concat](https://docs.rs/polars/latest/polars/prelude/fn.concat.html) to concatenate the `DataFrame`
in all groups into a single `DataFrame`.

```rust,ignore
pub fn read_row_groups(data: Bytes, file_metadata: &FileMetaData) -> Result<DataFrame> {
    todo!("step07: implement read row groups")
}
```

## Test

Test case for this step is `step07_row_group`.

## Hints and Solution

<details>
  <summary>Hint (how to get column chunk)</summary>

`RowGroup` has a member `columns` which contains a vector of `ColumnChunk`.

</details>

<details>
  <summary>Hint (How to concatenate multiple data frames)</summary>

To concatenate data frames, you can convert the `DataFrame` into a `LazyFrame`, then use
[`concat`](https://docs.rs/polars/latest/polars/prelude/fn.concat.html) function.

```rust,ignore
// convert `DataFrame` into `LazyFrame`
let lazyframes: Vec<LazyFrame> = dataframes.into_iter().map(|df| df.lazy()).collect();

// concatenate `LazyFrame` to a single `DataFrame`
concat(
    lazyframes,
    UnionArgs {
        strict: true,
        ..Default::default()
    },
)?
.collect()?;
```

</details>

<details>
  <summary>Solution</summary>

`read_row_group` function

```rust,ignore
pub fn read_row_group(data: Bytes, row_group: &RowGroup) -> Result<DataFrame> {
    let mut columns = Vec::with_capacity(row_group.columns.len());
    for column_chunk in &row_group.columns {
        let column = read_column(data.clone(), column_chunk)?;
        columns.push(column);
    }
    let df = DataFrame::new_infer_height(columns)?;
    Ok(df)
}
```

`read_row_groups` function

```rust,ignore
pub fn read_row_groups(data: Bytes, file_metadata: &FileMetaData) -> Result<DataFrame> {
    let mut dfs = Vec::with_capacity(file_metadata.row_groups.len());
    for row_group in &file_metadata.row_groups {
        let df = read_row_group(data.clone(), row_group)?;
        dfs.push(df.lazy());
    }
    let df = concat(
        dfs,
        UnionArgs {
            strict: true,
            ..Default::default()
        },
    )?
    .collect()?;
    Ok(df)
}
```

</details>
