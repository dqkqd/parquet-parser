# Magic Number

The magic number tells the parser whether it is reading a parquet file. This *number* is a 4-byte
`PAR1` and is located at the beginning and at the end of a file.

![magic number is located at the beginning and at the end of a file](images/magic-number-locations.png)

## Task

Implement the `ensure_header_footer_magic` function in `src/magic.rs`. It takes a whole file data in
`Bytes` and returns an error if the data is not a parquet file.

```rust,ignore
pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    todo!("step01: implement magic number")
}
```

## Test

To verify the implementation, uncomment the test in `tests/integration/mod.rs`.

```diff
-// mod step01_magic;
+mod step01_magic;
```

And run

```bash
cargo test
```

## Hints and Solution

<details>
  <summary>Hint</summary>

*There is no hint for this task.*

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn ensure_header_footer_magic(data: Bytes) -> Result<()> {
    if data.len() < 8 || !data.starts_with(b"PAR1") || !data.ends_with(b"PAR1") {
        bail!("Magic: not a parquet file")
    }
    Ok(())
}
```

</details>
