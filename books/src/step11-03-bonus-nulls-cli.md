# Bonus: Interactive Testing

The parser can now read data with NULL values. Let's test it!

## Data

The data is in `data/nulls.csv`.

```csv
col_i64,col_string
1,one
2,two
,
,
3,three
4,four
5,five
,
```

## Command

```bash
# write csv to a parquet file
cargo run write data/nulls.csv nulls.parquet

# read the parquet file
cargo run read nulls.parquet
```

## Result

You can see there are `null` values added.

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/parquet-parser read nulls.parquet`
shape: (8, 2)
┌─────────┬────────────┐
│ col_i64 ┆ col_string │
│ ---     ┆ ---        │
│ i64     ┆ str        │
╞═════════╪════════════╡
│ 1       ┆ one        │
│ 2       ┆ two        │
│ null    ┆ null       │
│ null    ┆ null       │
│ 3       ┆ three      │
│ 4       ┆ four       │
│ 5       ┆ five       │
│ null    ┆ null       │
└─────────┴────────────┘
```
