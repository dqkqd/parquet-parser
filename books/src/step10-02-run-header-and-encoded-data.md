# Run

In RLE Bit-packed hybrid encoding, multiple runs are packed together. In this step, we will extract
information for a single run using the decoded run header.

![RLE bit-packed hybrid format](images/rle-bit-packed-hybrid-format.png)

## Run header

A run header is **an integer** containing two pieces of information:

- An indicator whether a run is RLE or bit-packed: this is stored in the LSB, `0` if the run is a
  RLE run, otherwise it is a bit-packed run.
- The run length tells us how many values in the run: this is the remaining data without the LSB.

![A run can be rle or bit-packed](images/a-run-can-be-rle-or-bit-packed.png)

For example, if the run header is 13 (represented as `1101` in binary), then:

- This is a bit-packed run because LSB is 1.
- The run length is `13 >> 1 = 6`.

## RLE run

From the [spec](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE), a RLE run
has some properties:

- The LSB is 0.
- The run length is exactly the header's run length. This is the number of values for a RLE run.
- The repeated value is stored as the encoded value. The number of bytes for this repeated value is
  calculated by rounding the required bit-width to the next byte. For example, if the bit-width is
  1, then it needs 1 byte; if the bit-width is 9, then it needs 2 bytes.

![RLE run has LSB = 0, run length, and the repeated values](images/rle-run-breakdown-example.png)

## Bit-packed run

For a bit-packed run:

- The LSB is 1.
- The run length is the header's run length multiplied by 8. This could be referred to as the number
  of values in a bit-packed run (might contain garbage).
- The values are encoded using bit-packed encoding, the total **bits** needed is the bit-width
  multiplied with the run length.

![Bit-packed run has LSB = 1, run length and the bit-packed encoded value](images/bit-packed-run-breakdown-example.png)

> Because a bit-packed run stores multiple of 8 values, it might contain garbage. For example, if
> the bit-width is 1, and the number of values is 2, then the run stores 8 values (run length is 8),
> of which 6 are garbage.

## Code representation

We represent a run using an enum `RleBitPackedRun`. All members are the same as introduced in the
[RLE run](#rle-run) and the [Bit-packed run](#bit-packed-run) sections.

```rust,ignore
pub enum RleBitPackedRun {
    Rle {
        run_len: usize,
        bit_width: u8,
        repeated_value: Bytes,
    },
    BitPacked {
        run_len: usize,
        bit_width: u8,
        bit_packed_values: Bytes,
    },
}
```

## Task

Implement the `get_rle_bit_packed_run` function in `src/decoder/rle_bit_packing_hybrid.rs`. It takes
the encoded data (the packed multiple runs data) in `Bytes`, and returns the correct run with the
remaining bytes.

```rust,ignore
pub fn get_rle_bit_packed_run(
    encoded_data: Bytes,
    bit_width: u8,
) -> Result<(RleBitPackedRun, Bytes)> {
    todo!("step10-02: implement getting RleBitPackedRun")
}
```

## Test

Test case for this step is `step10_02_run_header_and_encoded_data`.

## Hints and Solution

<details>
    <summary>Hint (how to decode the header)</summary>

Use `uleb128_decode` function to extract the header.

</details>

<details>
    <summary>Hint (how to extract LSB and the run length)</summary>

```rust,ignore
let lsb = header & 1;
let length = header >> 1;
```

</details>

<details>
    <summary>Hint (how to extract RLE repeated value)</summary>

Calculate the bytes needed for the repeated value using the provided bit-width, then use this value
to extract the repeated value.

```rust,ignore
let needed_bytes = bit_width.div_ceil(8);
```

</details>

<details>
    <summary>Hint (how to extract bit-packed values)</summary>

Get the number of bytes needed for the run, then use this value to extract the encoded values.

```rust,ignore
let needed_bytes = run_len * bit_width / 8;
```

</details>

<details>
    <summary>Solution</summary>

```rust,ignore
pub fn get_rle_bit_packed_run(
    encoded_data: Bytes,
    bit_width: u8,
) -> Result<(RleBitPackedRun, Bytes)> {
    let (header, mut remaining) = uleb128_decode(encoded_data)?;
    let lsb = header & 1;
    let length = (header >> 1) as usize;

    let run = if lsb == 0 {
        let needed_bytes = bit_width.div_ceil(8) as usize;
        let repeated_value = remaining.split_to(needed_bytes);

        RleBitPackedRun::Rle {
            run_len: length,
            bit_width,
            repeated_value,
        }
    } else {
        let run_len = length * 8;
        let needed_bytes = run_len * bit_width as usize / 8;
        let bit_packed_values = remaining.split_to(needed_bytes);

        RleBitPackedRun::BitPacked {
            run_len,
            bit_width,
            bit_packed_values,
        }
    };

    Ok((run, remaining))
}
```

</details>
