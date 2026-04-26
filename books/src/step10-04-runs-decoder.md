# Runs Decoder

We have everything needed to decode the RLE Bit-packed encoding data. Again, the encoded data
contains multiple runs, each run can be either a RLE run or a Bit-packed run.

![RLE bit-packed hybrid format](images/rle-bit-packed-hybrid-format.png)

The length is optional and might not be included, this is documented in
[the spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE). However, since we
only focus on boolean values this time, the length is always included.

| Page kind    | RLE-encoded data kind | Prepend length? |
| ------------ | --------------------- | --------------- |
| Data page v1 | Definition levels     | Y               |
|              | Repetition levels     | Y               |
|              | Dictionary indices    | N               |
|              | Boolean values        | Y               |

## Task

You will implement three functions in this task: decoding a single run, then multiple runs, and
finally applying the RLE encoding to the parser.

### `decode_run`

Implement the `decode_run` function in `src/decoder/rle_bit_packing_hybrid.rs`. It takes a
`RleBitPackedRun` and returns a decoded vector of `Scalar`.

```rust,ignore
fn decode_run(run: RleBitPackedRun, parquet_type: Type) -> Result<Vec<Scalar>> {
    todo!("step10-04: implement decoding a single run")
}
```

### `rle_bit_packing_hybrid_decode`

Implement the `rle_bit_packing_hybrid_decode` function in `src/decoder/rle_bit_packing_hybrid.rs`.
It takes an encoded data and returns a decoded vector of `Scalar`. Depending on the `prepend_length`
variable, the length might not be included in the encoded data.

```rust,ignore
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    todo!("step10-04: implement decoding rle bit-packed encoded data")
}
```

> One caveat is that you need to handle the `num_values` correctly. The number of values here might
> not equal the total run length collected from all the runs (because a
> [bit-packed run might contain garbage](./step10-02-run-header-and-encoded-data.md#bit-packed-run)).

### `decode_data_page`

Handle the match arm `Encoding::RLE` in `src/decoder/mod.rs`. Again, the data type is always boolean
with 1 bit-width.

```rust,ignore
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
        // ...
        Encoding::RLE => todo!("step10-04: rle bit-packed hybrid decoder"),
        // ...
    }
}
```

## Test

Test case for this step is `step10_04_runs_decoder`.

## Hints and Solution

<details>
    <summary>Hint (how to decode a run)</summary>

Check the correct run type and apply appropriate decoder to the run.

</details>

<details>
    <summary>Hint (how to decode all the runs)</summary>

First, check whether the encoded data contains the length and take the actual encoded data out.

```rust,ignore
if prepend_length {
    // take the actual encoded data
}
```

Then keep extracting and decode each run separately until there is no remaining data left.

```rust,ignore
while !encoded_data.is_empty() {
    let (run, remaining) = get_rle_bit_packed_run(encoded_data, bit_width)?;
    // decode run
    encoded_data = remaining;
}
```

</details>

<details>
    <summary>Hint (how to handle the Encoding RLE arm)</summary>

There are three important points:

- The data is always boolean.
- The bit-width is always 1.
- The prepend_length is always true (Refer to the table in the spec).

Then call the `rle_bit_packing_hybrid_decode` with appropriate arguments.

</details>

<details>
    <summary>Solution</summary>

`decode_run`:

```rust,ignore
fn decode_run(run: RleBitPackedRun, parquet_type: Type) -> Result<Vec<Scalar>> {
    match run {
        RleBitPackedRun::Rle {
            run_len,
            bit_width,
            repeated_value,
        } => rle_decode(repeated_value, parquet_type, bit_width, run_len),
        RleBitPackedRun::BitPacked {
            run_len,
            bit_width,
            bit_packed_values,
        } => bit_packed_decode(bit_packed_values, parquet_type, bit_width, run_len),
    }
}
```

`rle_bit_packing_hybrid_decode`:

```rust,ignore
pub fn rle_bit_packing_hybrid_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
    prepend_length: bool,
) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;

    if prepend_length {
        let length = encoded_data.get_u32_le();
        encoded_data = encoded_data.slice(..length as usize);
    }

    let mut result = Vec::with_capacity(num_values);
    while !encoded_data.is_empty() {
        let (run, remaining) = get_rle_bit_packed_run(encoded_data, bit_width)?;
        let scalars = decode_run(run, parquet_type)?;
        result.extend(scalars);
        encoded_data = remaining;
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
