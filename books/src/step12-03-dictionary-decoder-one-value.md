# Dictionary Decoder (one value)

If all the values for a given column are the same, then:

- The dictionary page only has 1 value
- All the data pages store 0 as their bit-width and no encoded indexes are stored.

You should handle this case correctly to avoid panicking when there is no encoded indexes.

## Task

Update the `dictionary_decode` function in `src/decoder/dictionary.rs` to handle a bit-width of 0.

```rust,ignore
pub fn dictionary_decode(encoded_data: Bytes, num_values: usize) -> Result<Vec<Scalar>> {
    // ...
}
```

## Test

Test case for this step is `step12_03_dictionary_decoder_one_value`.

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
