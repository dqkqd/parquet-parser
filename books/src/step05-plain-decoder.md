# Plain Decoder

We now have all pages for a single column chunk. However, all of them are encoded. In this step, we
will decode page data and extract the actual column values. Let's start with the simplest one:
[Plain encoding](https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN).

![All values in plain encoding are encoded back to back separately](images/plain-encoding-encodes-values-back-to-back.png)

## Plain Encoding

In plain encoding, each value is encoded separately depending on the column data type. For our
parser, only these data types are supported (the explanations are taken from the
[spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN)).

| Data type | Parquet type | Explanation                                                                  |
| --------- | ------------ | ---------------------------------------------------------------------------- |
| BOOLEAN   | BOOLEAN      | Bit packed, LSB first                                                        |
| INT32     | INT32        | 4 bytes little endian                                                        |
| INT64     | INT64        | 8 bytes little endian                                                        |
| FLOAT     | FLOAT        | 4 bytes IEEE little endian                                                   |
| DOUBLE    | DOUBLE       | 8 bytes IEEE little endian                                                   |
| STRING    | BYTE_ARRAY   | length in 4 bytes little endian followed by the bytes contained in the array |

To represent a decoded value in code, we use
[Polars Scalar](https://docs.rs/polars/latest/polars/prelude/struct.Scalar.html). This makes the
implementation much simpler as we don't have to deal with type erasure, type casting, etc. A
`Scalar` can be created like this.

```rust,ignore
let scalar_integer = Scalar::from(1i32);
let scalar_string = Scalar::from(PlSmallStr::from_string("one"))
```

## Task

### `plain_decode`

Implement the `plain_decode` function in `src/decoder/plain.rs`. It takes the encoded page data as
`Bytes` and returns a decoded vector of `Scalar` based on the data type. The `num_values` is the
expected value for the vector.

```rust,ignore
pub fn plain_decode(
    encoded_data: Bytes,
    parquet_type: Type,
    num_values: usize,
) -> Result<Vec<Scalar>> {
    match parquet_type {
        Type::INT32 => todo!("step05: decode int32"),
        Type::INT64 => todo!("step05: decode int64"),
        Type::FLOAT => todo!("step05: decode float"),
        Type::DOUBLE => todo!("step05: decode double"),
        Type::BYTE_ARRAY => todo!("step05: decode string"),
        Type::BOOLEAN => todo!("step09: decode boolean"),
        _ => unimplemented!("plain_decode: unsupported data type {:?}", parquet_type),
    }
}
```

Some important notes:

- You don't have to handle `BOOLEAN` data yet, it requires different encoding, which will be covered
  in [Boolean Data section](./step09-boolean-data.md)
- To avoid messing with unicode data, we assume all `BYTE_ARRAY` data can be converted to `String`
  without error. In other words, this never panics

  ```rust,ignore
  String::from_utf8(data).unwrap()
  ```

### `decode_page`

Implement the `decode_page` function in`src/decoder/mod.rs`. This is a wrapper around all supported
decoders, it checks the page's encoding and applies the correct decoder. You need to handle the
`Encoding::PLAIN` arm in this step.

```rust,ignore
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
        Encoding::PLAIN => todo!("step05: plain decoder"),
        // ...
    }
}
```

You can get the encoded page data using `Page::encoded_values()`.

## Test

Test case for this step is `step05_plain_decoder`.

## Hints and Solution

<details>
  <summary>Hint (how to decode non-string types)</summary>

Some functions from the [bytes crate docs](https://docs.rs/bytes/latest/bytes/index.html) are useful
to extract primitive types. The extracted value can be converted to `Scalar` using `Scalar::from`.
For example, this decodes the `INT32` data.

```rust,ignore
let scalar = Scalar::from(data.get_i32_le());
```

</details>

<details>
  <summary>Hint (how to decode string type)</summary>

String uses a variable length, the first 4 bytes is the length followed by the actual string value.

```rust,ignore
let length = data.get_u32_le() as usize;
let string = data.slice(..length)
```

The actual bytes value can then be converted to `String` using `String::from_utf8` and
`PlSmallStr::from_string`.

```rust,ignore
let string = String::from_utf8(data).unwrap();
Scalar::from(PlSmallStr::from_string(string))
```

</details>

<details>
  <summary>Solution</summary>

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
        Type::INT32 => {
            for _ in 0..num_values {
                scalars.push(Scalar::from(encoded_data.get_i32_le()))
            }
        }
        Type::INT64 => {
            for _ in 0..num_values {
                scalars.push(Scalar::from(encoded_data.get_i64_le()))
            }
        }
        Type::FLOAT => {
            for _ in 0..num_values {
                scalars.push(Scalar::from(encoded_data.get_f32_le()))
            }
        }
        Type::DOUBLE => {
            for _ in 0..num_values {
                scalars.push(Scalar::from(encoded_data.get_f64_le()))
            }
        }
        Type::BYTE_ARRAY => {
            for _ in 0..num_values {
                let size = encoded_data.get_u32_le() as usize;
                let string = String::from_utf8(encoded_data.split_to(size).to_vec())?;
                scalars.push(Scalar::from(PlSmallStr::from_string(string)))
            }
        }
        Type::BOOLEAN => todo!("step09: decode boolean"),
        _ => unimplemented!("plain_decode: unsupported data type {:?}", parquet_type),
    }

    Ok(scalars)
}
```

`decode_page`:

```rust,ignore
pub fn decode_page(page: &Page, parquet_type: Type, num_values: usize) -> Result<Vec<Scalar>> {
    match page.encoding() {
        Encoding::PLAIN => plain_decode(page.encoded_values(), parquet_type, num_values),
        // ...
    }
}
```

</details>
