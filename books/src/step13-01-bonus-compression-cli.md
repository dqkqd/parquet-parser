# Bonus: Interactive Testing

This time, we can read a real parquet file (Not those created from our CLI).

## Data

The data is the [titanic dataset](https://huggingface.co/datasets/BIT/titanic-dataset):

```bash
curl -L -o titanic.parquet "https://huggingface.co/datasets/BIT/titanic-dataset/resolve/refs%2Fconvert%2Fparquet/default/train/0000.parquet"
```

## Command

```bash
cargo run read titanic.parquet
```

## Result

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/parquet-parser read titanic.parquet`
shape: (891, 15)
┌──────────┬────────┬────────┬──────┬───┬──────┬─────────────┬───────┬───────┐
│ survived ┆ pclass ┆ sex    ┆ age  ┆ … ┆ deck ┆ embark_town ┆ alive ┆ alone │
│ ---      ┆ ---    ┆ ---    ┆ ---  ┆   ┆ ---  ┆ ---         ┆ ---   ┆ ---   │
│ i64      ┆ i64    ┆ str    ┆ f64  ┆   ┆ str  ┆ str         ┆ str   ┆ bool  │
╞══════════╪════════╪════════╪══════╪═══╪══════╪═════════════╪═══════╪═══════╡
│ 0        ┆ 3      ┆ male   ┆ 22.0 ┆ … ┆ null ┆ Southampton ┆ no    ┆ false │
│ 1        ┆ 1      ┆ female ┆ 38.0 ┆ … ┆ C    ┆ Cherbourg   ┆ yes   ┆ false │
│ 1        ┆ 3      ┆ female ┆ 26.0 ┆ … ┆ null ┆ Southampton ┆ yes   ┆ true  │
│ 1        ┆ 1      ┆ female ┆ 35.0 ┆ … ┆ C    ┆ Southampton ┆ yes   ┆ false │
│ 0        ┆ 3      ┆ male   ┆ 35.0 ┆ … ┆ null ┆ Southampton ┆ no    ┆ true  │
│ …        ┆ …      ┆ …      ┆ …    ┆ … ┆ …    ┆ …           ┆ …     ┆ …     │
│ 0        ┆ 2      ┆ male   ┆ 27.0 ┆ … ┆ null ┆ Southampton ┆ no    ┆ true  │
│ 1        ┆ 1      ┆ female ┆ 19.0 ┆ … ┆ B    ┆ Southampton ┆ yes   ┆ true  │
│ 0        ┆ 3      ┆ female ┆ null ┆ … ┆ null ┆ Southampton ┆ no    ┆ false │
│ 1        ┆ 1      ┆ male   ┆ 26.0 ┆ … ┆ C    ┆ Cherbourg   ┆ yes   ┆ true  │
│ 0        ┆ 3      ┆ male   ┆ 32.0 ┆ … ┆ null ┆ Queenstown  ┆ no    ┆ true  │
└──────────┴────────┴────────┴──────┴───┴──────┴─────────────┴───────┴───────┘
```

## Metadata

If you inspect the metadata, you can see that the compression is snappy!

```bash
cargo run metadata titanic.parquet
```

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.17s
     Running `target/debug/parquet-parser metadata titanic.parquet`
...
column 0:
--------------------------------------------------------------------------------
column type: INT64
column path: "survived"
encodings: PLAIN RLE RLE_DICTIONARY
file path: N/A
file offset: 228
num of values: 891
compression: SNAPPY
total compressed size (in bytes): 224
total uncompressed size (in bytes): 219
...
```
