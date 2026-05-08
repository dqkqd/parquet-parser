# Dictionary Decoder

In the upcoming steps, we decode pages with
[Dictionary Encoding](https://parquet.apache.org/docs/file-format/data-pages/encodings/#DICTIONARY).

## Dictionary Encoding

Dictionary encoding encodes data in 2 places:

- A dictionary page: stores actual values encoded using [Plain Encoding](./step05-plain-decoder.md),
  which we have fully implemented
- Multiple Data pages: store value's indexes (in integers) encoded using
  [RLE Bit-packing Hybrid Encoding](./step10-rle-bit-packing-hybrid-decoder-boolean.md). However,
  the RLE Bit-packing Hybrid decoder can only work with booleans at the moment

![Dictionary page stores values, data page stores indexes](images/dictionary-page-stores-values-data-pages-store-indexes.png)

## Implementation

The implementation flow is a bit unusual. We first teach the parser about dictionary page, then
parse columns in dictionary encoding with at most two unique values, and finally make them work with
arbitrary number of values.
