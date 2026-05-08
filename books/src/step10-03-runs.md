# Runs

In this step, we will extract all the runs for a page encoded using RLE Bit-packing Hybrid Encoding.

![RLE bit-packed hybrid format](images/rle-bit-packed-hybrid-format.png)

The length is optional and might not be included, this is documented in
[the spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE).

| Page kind    | RLE-encoded data kind | Prepend length? |
| ------------ | --------------------- | --------------- |
| Data page v1 | Definition levels     | Y               |
|              | Repetition levels     | Y               |
|              | Dictionary indices    | N               |
|              | Boolean values        | Y               |

## Task

Implement the `read_rle_bit_packed_runs` function in `src/decoder/rle_bit_packing_hybrid.rs`. It
takes the encoded page and returns all the runs.

```rust,ignore
pub fn read_rle_bit_packed_runs(
    encoded_data: Bytes,
    bit_width: u8,
    prepend_length: bool,
) -> Result<Vec<RleBitPackedRun>> {
    todo!("step10-03: extract all runs")
}
```

You need to check the `prepend_length` argument to correctly extract the multiple packed runs.

## Test

Test case for this step is `step10_03_runs`.

## Hints and Solution

<details>
  <summary>Hint (how to get all the runs)</summary>

Similar to the [Data Page section](./step04-data-pages.md), you should extract the run from the
encoded data until there are no bytes left.

```rust,ignore
while !page_data.is_empty() {
    let (run, remaining) = read_rle_bit_packed_run(/* ... */);
    // ...
    page_data = remaining;
}
```

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn read_rle_bit_packed_runs(
    encoded_data: Bytes,
    bit_width: u8,
    prepend_length: bool,
) -> Result<Vec<RleBitPackedRun>> {
    let mut encoded_data = encoded_data;

    if prepend_length {
        let length = encoded_data.get_u32_le();
        encoded_data = encoded_data.slice(..length as usize);
    }

    let mut runs = Vec::new();
    while !encoded_data.is_empty() {
        let (run, remaining) = read_rle_bit_packed_run(encoded_data, bit_width)?;
        runs.push(run);
        encoded_data = remaining;
    }

    Ok(runs)
}

```

</details>
