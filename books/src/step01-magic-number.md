# Magic Number

The magic number tells the parser whether it is reading a parquet file. This is a 4-byte `PAR1` and
is located at the beginning and at the end of a file.

![magic number is located at the beginning and at the end of a file](images/magic-number-locations.png)

## Task

Implement the `ensure_header_footer_magic` function in `src/magic.rs`. It takes the entire file data
as `Bytes` and returns an error if this is not a parquet file.

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

And run:

```bash
cargo test
```

## Hints and Solution

<details>
  <summary>Hint</summary>

You can use `starts_with` and `ends_with` functions to check whether the magic number is correctly
present at both ends of the file.

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
