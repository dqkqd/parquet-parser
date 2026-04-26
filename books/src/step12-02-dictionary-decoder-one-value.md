# Dictionary Decoder (one value)

If the dictionary page only contains one value, all the values must be the same. In this case,
bit-width is 0 and no encoded indexes need to be stored in the data page.

## Task

Update the `dictionary_decode` function in `src/decoder/dictionary.rs` to handle a bit-width of 0.

```rust,ignore
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    // ...
}
```

## Test

Test case for this step is `step12_02_dictionary_decoder_one_value`.

## Hints and Solution

<details>
<summary>Solution</summary>

```rust,ignore
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    let mut encoded_data = encoded_data;
    let bit_width = encoded_data.get_u8();
    if bit_width == 0 {
        return Ok(vec![Scalar::from(0); num_values]);
    }
    rle_bit_packing_hybrid_decode(encoded_data, Type::INT32, bit_width, num_values, false)
}
```

</details>
