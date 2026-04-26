# RLE Hybrid Decoder (Boolean)

In the upcoming tasks, you will implement the
[RLE hybrid decoder](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE), which
supports these types of data:

- Repetition and definition levels
- Dictionary indices
- Boolean values in data pages, as an alternative to PLAIN encoding

We will eventually tackle all of them, but let's first go with the boolean values (because it is the
easiest one).

## RLE hybrid encoded data

A RLE hybrid encoded data contains a 4-byte length and many encoded runs, each run can be either a
RLE run or a Bit-packed run.

todo: figure RLE hybrid runs

## Run

A run contains a header and encoded values. The run header is encoded using
[ULEB-128 encoding](https://en.wikipedia.org/wiki/LEB128), and includes two important information:
the run length and the run type (RLE or Bit-packed)

todo: figure: run header and encoded values

## Implementation

This is a rough implementation guideline, we start with decoding the run header, then the run
values, and finally the hybrid encoded data with many runs. After that, we should be able to read a
parquet file with RLE encoding.
