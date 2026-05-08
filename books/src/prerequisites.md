# Prerequisites

## How to start

To start, clone [this repository](https://github.com/dqkqd/parquet-parser/), and checkout the
`starter` branch.

```bash
git clone https://github.com/dqkqd/parquet-parser.git

cd parquet-parser

git checkout starter
```

## How to test

Each step has several test cases in `tests/integration/mod.rs`, all of them are disabled by default.
You should uncomment the correct tests when implementing a specific step.

```rust,ignore
// mod step01_magic;
// mod step02_file_metadata;
// mod step03_data_page;
// mod step04_data_pages;
// mod step05_plain_decoder;
// mod step06_column;
// mod step07_row_group;
// mod step08_parquet_file;
// mod step09_boolean_column;
// ...
```

## Tips

- The codebase relies heavily on external crates such as
  [bytes](https://docs.rs/bytes/latest/bytes/); consider checking their docs when implementing.
- Having a look at the corresponding tests before implementation is always a good idea to understand
  what they actually test for.
