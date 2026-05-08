# Runs Decoder

We have everything we need to decode the RLE Bit-packing Hybrid encoded data. Let's apply it to our
parser.

![RLE bit-packed hybrid format](images/rle-bit-packed-hybrid-format.png)

For now we only
[handle the encoded boolean values](./step10-rle-bit-packing-hybrid-decoder-boolean.md); this means
the length must be included.

| Page kind    | RLE-encoded data kind | Prepend length? |
| ------------ | --------------------- | --------------- |
| Data page v1 | Definition levels     | Y               |
|              | Repetition levels     | Y               |
|              | Dictionary indices    | N               |
|              | **Boolean values**    | **Y**           |

## Task

### `rle_bit_packing_hybrid_decode`

Implement the `rle_bit_packing_hybrid_decode` function in `src/decoder/rle_bit_packing_hybrid.rs`.
It takes the encoded page data and returns a decoded vector of `Scalar`.

```rust,ignore
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    todo!("step10-05: decode all runs")
}
```

Because [a bit-packed run might contain garbage](./step10-02-run.md#bit-packed-run), the
`num_values` might not equal the total number of values in all pages.

### `decode_page`

Update the match arm `Encoding::RLE` in `src/decoder/mod.rs`. Again, the data type is always boolean
with 1 bit-width.

```rust,ignore
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
        // ...
        Encoding::RLE => todo!("step10-05: rle bit-packing hybrid decoder"),
        // ...
    }
}
```

## Test

Test case for this step is `step10_05_runs_decoder`.

## Hints and Solution

<details>
    <summary>Hint (steps)</summary>

- Extract all the runs
- Decode each run and concatenate the result
- Handle the number of values in the final result

</details>

<details>
    <summary>Solution</summary>

`rle_bit_packing_hybrid_decode`:

```rust,ignore
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    let runs = read_rle_bit_packed_runs(encoded_data, bit_width, prepend_length)?;
    let mut result = Vec::with_capacity(num_values);
    for run in runs {
        let scalars = rle_bit_packing_hybrid_run_decode(run, parquet_type)?;
        result.extend(scalars);
    }
    result.truncate(num_values);
    Ok(result)
}
```

`decode_data_page`:

```rust,ignore
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
        // ...
        Encoding::RLE => {
            rle_bit_packing_hybrid_decode(page.encoded_values(), parquet_type, 1, num_values, true)
        }
        // ...
    }
}
```

</details>
