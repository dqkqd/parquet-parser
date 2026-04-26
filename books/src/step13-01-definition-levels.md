# Definition Levels

Definition levels data is stored in
[Data Page](https://parquet.apache.org/docs/file-format/data-pages/), this is basically a boolean
vector encoded using
[RLE Bit-packed Hybrid Encoding](./step10-rle-bit-packing-hybrid-decoder-boolean.md), where `true`
means the value is present, `false` otherwise.

## Task

You will handle reading the definition levels data from a `DataPage` and decode it into a null map.

### `read_page`

Update the `read_page` function in `src/page.rs` to collect the definition levels data (you might
skip this if you have already handled the definition levels data correctly in
[Data Page](./step03-data-page.md)). Note that the definition levels data contains the length
itself.

```rust,ignore
pub fn read_page(data: Bytes, codec: CompressionCodec) -> Result<(Page, Bytes)> {
    // ...
}
```

### `decode_definition_levels`

Implement the `decode_definition_levels` function in `src/nulls.rs`. It takes a data page and
returns a null map.

```rust,ignore
pub fn decode_definition_levels(page: &Page) -> Result<Vec<bool>> {
    todo!("step13-01: decode definition levels")
}
```

## Test

Test case for this step is `step13_01_definition_levels`.

## Hints and Solution

<details>
  <summary>Hint (how to parse definition levels data)</summary>

Refer to the [hint section in Data Page](./step03-data-page.md#hints-and-solution).

</details>

<details>
  <summary>Solution</summary>

`read_page`:

```rust,ignore
pub fn read_page(data: Bytes, codec: CompressionCodec) -> Result<(Page, Bytes)> {
    let (page_header, mut remaining) = read_thrift_metadata::<PageHeader>(data)?;
    let mut page_data = remaining.split_to(page_header.compressed_page_size as usize);

    let page = match page_header.type_ {
        PageType::DATA_PAGE => {
            // because the definition levels contains the length itself,
            // we need to clone the data to avoid shifting its bytes.
            let definition_levels_len = page_data.clone().get_u32_le() as usize;
            let definition_levels = page_data.split_to(definition_levels_len + 4);

            Page::DataPage {
                page_header,
                definition_levels,
                encoded_values: page_data,
            }
        }
        PageType::DICTIONARY_PAGE => Page::DictionaryPage {
            page_header,
            encoded_values: page_data,
        },
        page_type => unimplemented!("read_page: unsupported {:?}", page_type),
    };

    Ok((page, remaining))
}
```

`decode_definition_levels`:

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
