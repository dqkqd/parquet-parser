# Bonus: Read a Parquet File from CLI

The parser can read real parquet files with plain encoding, no compression, etc. Let's see it in
action.

## Writer CLI

Parquet files with only plain encoding, no compression, etc. are pretty rare, so the starter code
comes with a CLI that can convert CSV files into parquet files satisfying such requirements. It can
be used like this (the default arguments creates a plain encoding with no compression parquet file).

```bash
cargo run --bin write <input-csv-file> <output-parquet-file>
```

## Try it out

Let's check if the current parser can work with such files. We will download
[this csv file](https://raw.githubusercontent.com/tobilg/public-cloud-provider-ip-ranges/bda4bc1ac501f8bab9cd618b47eb336328e732cc/data/providers/all.csv),
convert it to parquet, and then read it using our parser.

```bash
# download csv file
wget https://raw.githubusercontent.com/tobilg/public-cloud-provider-ip-ranges/bda4bc1ac501f8bab9cd618b47eb336328e732cc/data/providers/all.csv

# convert to parquet
cargo run --bin write all.csv all.parquet

# read the file
cargo run all.parquet
```

This is the result.

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
