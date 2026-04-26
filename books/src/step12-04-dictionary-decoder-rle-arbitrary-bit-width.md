# RLE arbitrary bit-width

This task handles RLE for an arbitrary bit-width.

## Task

Update the `rle_decode` function in `src/decoder/rle.rs` to handle arbitrary bit-width.

```rust,ignore
pub fn rle_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    // ...
}
```

## Test

Test case for this step is `step12_04_dictionary_decoder_rle`.

## Hints and Solution

<details>
  <summary>Hint</summary>

Use the `bit_packed_decode` function.

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn rle_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let scalar = bit_packed_decode(encoded_data, parquet_type, bit_width, 1)?
        .pop()
        .with_context(|| "rle_decode: cannot get decoded scalar from `bit_packed_decode`")?;
    let scalars = vec![scalar; num_values];
    Ok(scalars)
}
```

</details>
