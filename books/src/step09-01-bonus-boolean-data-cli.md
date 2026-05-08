# Bonus: Interactive Testing

The parser can now read boolean data. Let's test it!

## Data

The CSV data is in `data/boolean.csv`.

```csv
column
true
false
true
true
false
false
true
true
```

## Command

```bash
# convert to parquet
cargo run write data/boolean.csv boolean.parquet

# read it
cargo run read boolean.parquet
```

## Result

```bash
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
     Running `target/debug/parquet-parser read boolean.parquet`
shape: (8, 1)
┌────────┐
│ column │
│ ---    │
│ bool   │
╞════════╡
│ true   │
│ false  │
│ true   │
│ true   │
│ false  │
│ false  │
│ true   │
│ true   │
└────────┘
```
