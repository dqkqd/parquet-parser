# Bonus: Test with CLI

Since our parser can read a complete parquet file, let's see it in action. This section introduces
an interactive way to test it: Using CLI.

## CLI

The starter code comes with a CLI that has several useful commands:

```bash
Usage: parquet-parser <COMMAND>

Commands:
  read      Read parquet file
  write     Write a csv file to a parquet file
  metadata  Print the metadata for a parquet file
  verify    Verify the current parser output with the official parquet parser
  help      Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help
```

At the moment, the parser can only read parquet files with plain encoding, no compression, such
files are rare in the wild. The `write` command is a convenient way to convert CSV into parquet (its
default arguments generates a plain encoding with no compression parquet file).

```bash
Write a csv file to a parquet file

Usage: parquet-parser write [OPTIONS] <CSV> <PARQUET>

Arguments:
  <CSV>      The input csv file
  <PARQUET>  The output parquet file

Options:
      --author <AUTHOR>
          The author [default: "Hello parquet!"]
      --dictionary
          Whether to enable dictionary encoding
      --encodings <ENCODINGS>
          Encoding for each column. Syntax: `--encodings <column_name>=<encoding>`. Supported encodings: [rle]
      --compression <COMPRESSION>
          Compression for the output parquet [default: uncompressed] [possible values: uncompressed, snappy]
      --rows-per-page <ROWS_PER_PAGE>
          The number of row per page in a column chunk
      --rows-per-group <ROWS_PER_GROUP>
          The number of row per groups in a row group
      --dtypes <DTYPES>
          Data type for each column. Syntax: `--dtypes <column_name>=<data_type>`. Supported data types: [boolean, int32, int64, float, double, string]
  -h, --help
          Print help
```

## Try it out

Let's convert
[this csv file](https://raw.githubusercontent.com/tobilg/public-cloud-provider-ip-ranges/bda4bc1ac501f8bab9cd618b47eb336328e732cc/data/providers/all.csv),
to parquet, and read it using our parser.

```bash
# download csv file
curl -L -o public-cloud-provider-ip-ranges.csv https://raw.githubusercontent.com/tobilg/public-cloud-provider-ip-ranges/bda4bc1ac501f8bab9cd618b47eb336328e732cc/data/providers/all.csv

# convert to parquet
cargo run write public-cloud-provider-ip-ranges.csv public-cloud-provider-ip-ranges.parquet

# read it
cargo run read public-cloud-provider-ip-ranges.parquet
```

### Result

This is the result

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/parquet-parser all.parquet`
shape: (62_689, 6)
┌────────────────┬─────────────────┬──────────────┬─────────────────┬────────────────┬────────────────┐
│ cloud_provider ┆ cidr_block      ┆ ip_address   ┆ ip_address_mask ┆ ip_address_cnt ┆ region         │
│ ---            ┆ ---             ┆ ---          ┆ ---             ┆ ---            ┆ ---            │
│ str            ┆ str             ┆ str          ┆ i64             ┆ i64            ┆ str            │
╞════════════════╪═════════════════╪══════════════╪═════════════════╪════════════════╪════════════════╡
│ AWS            ┆ 1.178.1.0/24    ┆ 1.178.1.0    ┆ 24              ┆ 256            ┆ us-west-2      │
│ AWS            ┆ 1.178.10.0/24   ┆ 1.178.10.0   ┆ 24              ┆ 256            ┆ eu-central-1   │
│ AWS            ┆ 1.178.100.0/24  ┆ 1.178.100.0  ┆ 24              ┆ 256            ┆ us-west-1      │
│ AWS            ┆ 1.178.101.0/24  ┆ 1.178.101.0  ┆ 24              ┆ 256            ┆ ap-northeast-3 │
│ AWS            ┆ 1.178.102.0/24  ┆ 1.178.102.0  ┆ 24              ┆ 256            ┆ ap-southeast-5 │
│ …              ┆ …               ┆ …            ┆ …               ┆ …              ┆ …              │
│ Vultr          ┆ 95.179.208.0/20 ┆ 95.179.208.0 ┆ 20              ┆ 4096           ┆ FR-93          │
│ Vultr          ┆ 95.179.224.0/20 ┆ 95.179.224.0 ┆ 20              ┆ 4096           ┆ GB-LND         │
│ Vultr          ┆ 95.179.240.0/20 ┆ 95.179.240.0 ┆ 20              ┆ 4096           ┆ DE-HE          │
│ Vultr          ┆ 96.30.192.0/20  ┆ 96.30.192.0  ┆ 20              ┆ 4096           ┆ US-GA          │
│ Vultr          ┆ 96.30.208.0/20  ┆ 96.30.208.0  ┆ 20              ┆ 4096           ┆ US-FL          │
└────────────────┴─────────────────┴──────────────┴─────────────────┴────────────────┴────────────────┘
```

### Metadata

You can also inspect the metadata using the `metadata` command:

```bash
cargo run metadata public-cloud-provider-ip-ranges.parquet
```

The result is quite verbose, but we can see that all of the columns are encoded using plain
encoding. (The `RLE` is for definition levels, )

```diff
...
version: 1
num of rows: 62689
created by: Hello parquet!
...

column 0:
--------------------------------------------------------------------------------
column type: BYTE_ARRAY
column path: "cloud_provider"
encodings: PLAIN RLE
...

column 1:
--------------------------------------------------------------------------------
column type: BYTE_ARRAY
column path: "cidr_block"
encodings: PLAIN RLE
...

column 2:
--------------------------------------------------------------------------------
column type: BYTE_ARRAY
column path: "ip_address"
encodings: PLAIN RLE
...

column 3:
--------------------------------------------------------------------------------
column type: INT64
column path: "ip_address_mask"
encodings: PLAIN RLE
...

column 4:
--------------------------------------------------------------------------------
column type: INT64
column path: "ip_address_cnt"
encodings: PLAIN RLE
...

column 5:
--------------------------------------------------------------------------------
column type: BYTE_ARRAY
column path: "region"
encodings: PLAIN RLE
...
```
