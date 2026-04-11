use bytes::Bytes;
use parquet_parser::magic::ensure_header_footer_magic;

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
fn header_or_footer_is_missing() {
    let data = Bytes::from("PAR1");
    assert!(ensure_header_footer_magic(data).is_err());
}

#[test]
fn empty_data() {
    let data = Bytes::from("");
    assert!(ensure_header_footer_magic(data).is_err());
}
