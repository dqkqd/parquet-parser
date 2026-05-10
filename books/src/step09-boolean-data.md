# Boolean data

This section will handle the boolean data type. Recall from the
[Plain Decoder section](./step05-plain-decoder.md), boolean data type is encoded using
[bit-packed encoding](https://parquet.apache.org/docs/file-format/data-pages/encodings/#BITPACKED).

| Data type | Parquet type | Explanation           |
| --------- | ------------ | --------------------- |
| BOOLEAN   | BOOLEAN      | Bit packed, LSB first |

## Bit-packed encoding

Bit-packed encoding encodes each value into bits (using the same bit-width), then packs them
together (hence the name bit-packed). Below is an example of encoding 10, 20, 30, 40 using 6-bit
width.

![bit-packed encodes data in general](images/bit-packed-general.png)

*The figure above just gives you a rough idea of how bit-packed works in general, it isn't exactly
what parquet bit-packed encoding does, we will look into this later in
[Bit-packed arbitrary bit-width](./step12-04-dictionary-decoder-bit-packed-arbitrary-bit-width.md).*

## Parquet bit-packed encoding for boolean data

For boolean data, each value can be either `true` or `false`, so 1-bit width is sufficient. Encoding
and decoding using 1-bit width is much easier than arbitrary bit-width because there are no values
crossing byte boundaries.

### Encode

For encoding, values are packed together into 8-bit groups using LSB (Least Significant Bit) first.
Groups with fewer than 8 bits are padded with 0.

![bit-packed encoding animation](./images/bit-packed-animation/encode/output.gif)

### Decode

Decoding can be performed by fetching every 8-bit group at a time, then shifting bits until there is
no remaining data left (or if we get enough values).

![bit-packed decoding animation](./images/bit-packed-animation/decode/output.gif)

> You can optimize decoding by fetching more than 8 bits at a time (i.e. 32 bits).

## Task

### `bit_packed_decode`

Implement the `bit_packed_decode` function in `src/decoder/bit_packed.rs`. It takes the encoded page
data as `Bytes` and returns a decoded vector of `Scalar`.

```rust,ignore
pub fn bit_packed_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    todo!("step09: implement the boolean data decoder")
}
```

For boolean data, the bit-width is always 1.

### `plain_decode`

Update the `plain_decode` in `src/decoder/plain.rs` function to handle boolean data type.

```rust,ignore
pub fn plain_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    match parquet_type {
        // ...
        Type::BOOLEAN => todo!("step09: decode boolean"),
        // ...
    }
}
```

## Test

Test case for this step is `step09_boolean_column`.

## Hints and Solution

<details>
    <summary>Hint (decoding steps)</summary>

- Fetch the data each 8 bits at a time. (You can optimize by reading 4 bytes at a time in little
  endian).
- Shift right until there are no bits left or until you get enough values.
- Create vector of boolean `Scalar`.

</details>

<details>
    <summary>Solution</summary>

`bit_packed_decode`:

```rust,ignore
pub fn bit_packed_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    bit_width: u8,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let mut needed = num_values;
    let mut scalars = Vec::with_capacity(num_values);
    while needed > 0 {
        let group = encoded_data.get_u8();
        for i in 0..needed.min(8) {
            scalars.push(Scalar::from(group >> i & 1 == 1));
        }
        needed = needed.saturating_sub(8);
    }
    Ok(scalars)
}
```

`plain_decode`:

```rust,ignore
pub fn plain_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let mut scalars = Vec::with_capacity(num_values);

    match parquet_type {
        // ...
        Type::BOOLEAN => scalars = bit_packed_decode(encoded_data, Type::BOOLEAN, 1, num_values)?,
        // ...
    }

    Ok(scalars)
}
```

</details>
