use anyhow::Result;
use bytes::Bytes;
use parquet::basic::{Compression, Encoding};
use parquet_parser::magic::ensure_header_footer_magic;

use crate::make_parquet;

#[test]
fn correct_header_and_footer() {
    let data = Bytes::from("PAR1xyzPAR1");
    assert!(ensure_header_footer_magic(data).is_ok());
}

#[test]
fn missing_header_and_footer() {
    let data = Bytes::from("xyz");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn missing_header() {
    let data = Bytes::from("xyzPAR1");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn missing_footer() {
    let data = Bytes::from("PAR1xyz");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn empty() {
    let data = Bytes::from("");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn header_and_footer_duplicated() {
    let data = Bytes::from("PAR1");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn real_parquet_file() -> Result<()> {
    let data = make_parquet(
        r#"
col
1
2
3
"#,
        false,
        Encoding::PLAIN,
        Compression::UNCOMPRESSED,
    )?;

    assert!(ensure_header_footer_magic(data).is_ok());
    Ok(())
}
