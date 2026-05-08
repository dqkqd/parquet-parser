# Run Decoder

In this step, we will decode an RLE Bit-packing Hybrid run.

![RLE bit-packing hybrid run with header and encoded values](images/rle-bit-packing-hybrid-run.png)

As you might have guessed, we need 2 decoders, one for an RLE run, and the other for a bit-packed
run. Since we have already handled the latter in [Boolean Data section](./step09-boolean-data.md),
we only need to implement an RLE decoder.

## RLE decoder

Recall from the [RLE definition](./step10-rle-bit-packing-hybrid-decoder-boolean.md#rle), an RLE run
contains a run length and repeated value. Decoding it is just duplicating the repeated value with
the run length.

![RLE encoding encodes runs into length and value](images/rle-in-general.png)

## Task

You will implement two functions for decoding an RLE run and an RLE Bit-packing Hybrid run.

### `rle_decode`

Implement the `rle_decode` function in `src/decoder/rle.rs`. It takes the encoded repeated value as
`Bytes` and returns a decoded vector of `Scalar`.

```rust,ignore
pub fn rle_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!("step10-04: decode a rle run")
}
```

The data type is always boolean with 1-bit width.

### `rle_bit_packing_hybrid_run_decode`

Implement the `rle_bit_packing_hybrid_run_decode` function in
`src/decoder/rle_bit_packing_hybrid.rs`. It takes a run and returns a decoded vector of `Scalar`.

```rust,ignore
pub fn rle_bit_packing_hybrid_run_decode(
    run: RleBitPackedRun,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    todo!("step10-04: decode a single run")
}
```

## Test

Test case for this step is `step10_04_run_decoder`.

## Hints and Solution

<details>
    <summary>Hint (how to decode an RLE run)</summary>

An RLE run is just a bit-packed run where the number of values is 1.

</details>

<details>
    <summary>Solution</summary>

`rle_decode`:

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

`rle_bit_packing_hybrid_run_decode`:

```rust,ignore
pub fn rle_bit_packing_hybrid_run_decode(
    run: RleBitPackedRun,
    parquet_type: Type,
) -> Result<Vec<Scalar>> {
    match run {
        RleBitPackedRun::Rle {
            run_len,
            bit_width,
            encoded_values,
        } => rle_decode(encoded_values, parquet_type, bit_width, run_len),
        RleBitPackedRun::BitPacked {
            run_len,
            bit_width,
            encoded_values,
        } => bit_packed_decode(encoded_values, parquet_type, bit_width, run_len),
    }
}
```

</details>
