# RLE Bit-packing Hybrid Decoder

Until now, the parser can only read files with
[PLAIN Encoding](https://parquet.apache.org/docs/file-format/data-pages/encodings/#PLAIN). In the
upcoming tasks, we will handle
[RLE Bit-packing Hybrid Encoding](https://parquet.apache.org/docs/file-format/data-pages/encodings/#RLE).
This will be much more fun (and harder)!

From the spec, this encoding supports the following types of data.

- Repetition and definition levels
- Dictionary indices
- Boolean values in data pages, as an alternative to PLAIN encoding

We will handle all of them eventually, but let's first go with the boolean values (because it is the
easiest). Let's look at some key terms before diving into implementation.

## RLE

The [previous chapter](./step09-boolean-data.html#bit-packed-encoding-general) explains the
bit-packed encoding. For the RLE (Run Length Encoding) the data is encoded using two pieces of
information: the run length and the repeated value.

![RLE encoding encodes runs into length and value](images/rle-in-general.png)

## RLE Bit-packing Hybrid Encoding

RLE Hybrid encoded data contains a 4-byte length and multiple encoded runs written back to back. As
the name suggests, each run can be either a RLE run or a Bit-packed run, which contains two parts: a
run header and the encoded values (both are themselves encoded!).

![RLE bit-packed hybrid format](images/rle-bit-packed-hybrid-format.png)

## Implementation

For a rough implementation guideline, we start bottom up by decoding the run header, then the run
encoded values, and finally the hybrid encoded data with multiple packed runs.
