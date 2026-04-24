use std::io::Cursor;

use anyhow::Result;
use bytes::Bytes;
use thrift::protocol::{TCompactInputProtocol, TSerializable};

pub fn read_thrift_metadata<T: TSerializable>(data: &mut Bytes) -> Result<T> {
    let mut cursor = Cursor::new(data.as_ref());
    let decoded = {
        let mut protocol = TCompactInputProtocol::new(&mut cursor);
        T::read_from_in_protocol(&mut protocol)?
    };
    *data = data.slice(cursor.position() as usize..);

    Ok(decoded)
}
