# Definition Levels Decoder

Definition levels data is stored in the [Data Page](./step03-data-page.md#data-page-layout). It is a
null map indicating which values exist in a page: `true` means the value is present, `false`
otherwise.

![NULLs in data pages and definition levels](images/nulls-data-pages-and-definition-levels.png)

The definition levels data itself is encoded using
[RLE Bit-packed Hybrid Encoding](./step10-rle-bit-packing-hybrid-decoder-boolean.md) for boolean
data, which we have already implemented in the previous steps! To represent the null map in code, we
simply use a vector of booleans.

## Task

Implement the `decode_definition_levels` function in `src/nulls.rs`. It takes a `Page::DataPage` and
returns a null map.

```rust,ignore
pub fn decode_definition_levels(page: &Page) -> Result<Vec<bool>> {
    todo!("step11-01: decode definition levels")
}
```

## Test

Test case for this step is `step11_01_definition_levels_decoder`.

## Hints and Solution

<details>
  <summary>Hint (how to convert a Scalar to boolean)</summary>

To convert a `Scalar` to boolean, you can use
[into_value](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html#method.into_value) and
[extract_bool](https://docs.rs/polars/latest/polars/prelude/enum.AnyValue.html#method.extract_bool)

```rust,ignore
let exist = scalar.into_value().extract_bool()
```

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn decode_definition_levels(page: &Page) -> Result<Vec<bool>> {
    let definition_levels = page
        .definition_levels()
        .with_context(|| "decode_definition_levels: receive non data page")?;

    let decoded_scalars = rle_bit_packing_hybrid_decode(
        definition_levels,
        Type::BOOLEAN,
        1,
        page.num_values(),
        true,
    )?;

    let is_present: Option<Vec<bool>> = decoded_scalars
        .into_iter()
        .map(|v| v.into_value().extract_bool())
        .collect();

    let is_present =
        is_present.with_context(|| "decode_definition_levels: invalid definition levels")?;

    Ok(is_present)
}
```

</details>
