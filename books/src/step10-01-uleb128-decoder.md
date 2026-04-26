# Run header decoder (ULEB128 Decoder)

A run header is **an integer** encoded using
[ULEB-128 encoding](https://en.wikipedia.org/wiki/LEB128#Unsigned_LEB128), Wikipedia has a very
well-explained encoding example for 624485.

```text
MSB ------------------ LSB
      10011000011101100101  In raw binary
     010011000011101100101  Padded to a multiple of 7 bits
 0100110  0001110  1100101  Split into 7-bit groups
00100110 10001110 11100101  Add high 1 bits on all but last (most significant) group to form bytes
    0x26     0x8E     0xE5  In hexadecimal

→ 0xE5 0x8E 0x26            Output stream (LSB to MSB)
```

## Decode

Decoding is the reverse operation: it takes the bytes, strips the MSB in each byte, then packs them
together into an integer.

```text
MSB ------------------ LSB
    0xE5     0x8E     0x26
11100101 10001110 00100110
 0100110  0001110  1100101  strip the leading bit
      10011000011101100101  group together
```

The tricky part is that actual data might contain redundant bytes (in our case, the run encoded
values after the header), which means the decoder must know when to stop fetching bytes.

```text
0xE5     0x8E     0x26     <redundant bytes>
```

This can be solved by looking at the MSB: because all groups except the last one have `1` at their
MSB, if the decoder sees a byte with leading `0`, it must stop.

```text
    0xE5     0x8E     0x26     <redundant bytes>
11100101 10001110 00100110     <remaining bits>
                  ^
          last byte for the header
```

## Task

Implement the `uleb128_decode` function in `src/decoder/uleb128.rs`. It takes encoded `Bytes` and
returns the decoded integer with the remaining bytes.

```rust,ignore
pub fn uleb128_decode(encoded_data: Bytes) -> Result<(u64, Bytes)> {
    todo!("step10-01: implement uleb128 decoder")
}
```

## Test

Test case for this step is `step10_01_uleb128_decoder`.

## Hints and Solution

<details>
  <summary>Hint</summary>

*There is no hint for this task.*

</details>

<details>
    <summary>Solution</summary>

```rust,ignore
pub fn uleb128_decode(encoded_data: Bytes) -> Result<(u64, Bytes)> {
    let mut encoded_data = encoded_data;
    let mut result = 0u64;

    let total_bytes = encoded_data.len();
    for i in 0..total_bytes {
        let byte = encoded_data.get_u8() as u64;
        result |= (byte & 0x7F) << (i * 7);
        // MSB = 0, stop
        if byte & 0x80 == 0 {
            return Ok((result, encoded_data));
        }
    }
    bail!("uleb128_decode: no byte with leading 0")
}
```

</details>
