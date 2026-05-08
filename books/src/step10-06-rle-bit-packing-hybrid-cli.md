# Bonus: Interactive Testing

The parser can now read data encoded with RLE Bit-packed Hybrid Encoding. Let's test it!

## Data

The CSV data is in `data/all.csv`.

```csv
col_bool,col_integer,col_real,col_string
true,1,1.1,one
false,2,2.2,two
true,3,3.3,three
true,4,4.4,four
false,5,5.5,five
false,6,6.6,six
true,7,7.7,seven
true,8,8.8,eight
```

## Command

To write to a parquet file using RLE Bit-packed Hybrid Encoding, set the encoding flag
`--encodings <COLUMN_NAME>=rle`

```bash
# write csv to a parquet file
cargo run write data/all.csv all.parquet --encodings col_bool=rle

# read the parquet file
cargo run read all.parquet
```

## Result

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/parquet-parser read all.parquet`
shape: (8, 4)
┌──────────┬─────────────┬──────────┬────────────┐
│ col_bool ┆ col_integer ┆ col_real ┆ col_string │
│ ---      ┆ ---         ┆ ---      ┆ ---        │
│ bool     ┆ i64         ┆ f64      ┆ str        │
╞══════════╪═════════════╪══════════╪════════════╡
│ true     ┆ 1           ┆ 1.1      ┆ one        │
│ false    ┆ 2           ┆ 2.2      ┆ two        │
│ true     ┆ 3           ┆ 3.3      ┆ three      │
│ true     ┆ 4           ┆ 4.4      ┆ four       │
│ false    ┆ 5           ┆ 5.5      ┆ five       │
│ false    ┆ 6           ┆ 6.6      ┆ six        │
│ true     ┆ 7           ┆ 7.7      ┆ seven      │
│ true     ┆ 8           ┆ 8.8      ┆ eight      │
└──────────┴─────────────┴──────────┴────────────┘
```

## Metadata

You can see from the metadata, the boolean column is encoded using RLE (no PLAIN included).

```bash
cargo run metadata all.parquet
```

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/parquet-parser metadata all.parquet`
...
column 0:
--------------------------------------------------------------------------------
column type: BOOLEAN
column path: "col_bool"
encodings: RLE
...

column 1:
--------------------------------------------------------------------------------
column type: INT64
column path: "col_integer"
encodings: PLAIN RLE
...
```
