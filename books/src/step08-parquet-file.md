# Parquet File

Let's put everything together and read a complete parquet file.

## Task

Implement the `read_parquet` function in `src/reader.rs`.

```rust,ignore
pub fn read_parquet(file_path: impl AsRef<Path>) -> Result<DataFrame> {
    todo!("step08: implement read parquet")
}
```

## Test

Test case for this step is `step08_parquet_file`.

## Hints and Solution

<details>
  <summary>Hint (steps to parse a parquet file)</summary>

- Read the file into `Vec<u8>` and convert it to `Bytes`
- Verify the magic number
- Read the file metadata
- Read the row groups

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
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
```

</details>
