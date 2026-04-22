use anyhow::Result;
use bytes::Bytes;
use parquet_parser::decoder::uleb128::uleb128_decode;

#[test]
fn ok() -> Result<()> {
    // Example from wikipedia: https://en.wikipedia.org/wiki/LEB128
    let mut data = Bytes::from(vec![0xE5, 0x8E, 0x26]);
    assert_eq!(data.len(), 3);
    assert_eq!(uleb128_decode(&mut data)?, 624485u64);
    assert_eq!(data.len(), 0);
    Ok(())
}

#[test]
fn data_does_not_contain_lsb_0() -> Result<()> {
    // Example from wikipedia: https://en.wikipedia.org/wiki/LEB128
    let mut data = Bytes::from(vec![0xE5, 0x8E, 0x8E]);
    assert!(uleb128_decode(&mut data).is_err());
    Ok(())
}
