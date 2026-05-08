# File Metadata

The parser needs to read some metadata before parsing column data. The first one is the file
metadata, located at the end of the file, right before the footer magic number.

![file metadata is located at the end of the file, before the footer magic number](images/file-metadata-position.png)

## Task

Implement the `read_file_metadata` function in `src/file_metadata.rs`. It takes the entire file data
as `Bytes` and returns a `FileMetaData` struct.

```rust,ignore
pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    todo!("step02: read file metadata.")
}
```

To parse it, you should read the 4-byte file metadata length first, then the raw file metadata, and
use the `read_thrift_metadata` introduced in the [Overview](./overview.md#metadata) to convert it to
`FileMetaData`.

## Test

The corresponding test is `step02_file_metadata`.

```diff
-// mod step02_file_metadata;
+mod step02_file_metadata;
```

## Hints and Solution

<details>
  <summary>Hint (how to get raw file metadata in bytes)</summary>

The 4-byte file metadata length can be parsed using
[`Bytes::get_u32_le`](https://docs.rs/bytes/latest/bytes/buf/trait.Buf.html#method.get_u32_le).
Remember it is right before the footer magic number.

```rust,ignore
let metadata_size = data.slice(data.len() - 8..).get_u32_le();
```

Then the raw file metadata in bytes can be extracted like this.

```rust,ignore
let metadata_bytes = data.slice(data.len() - 8 - metadata_size..data.len() - 8);
```

</details>

<details>
  <summary>Hint (how to parse file metadata)</summary>

The `FileMetaData` can be parsed using `read_thrift_metadata`.

```rust,ignore
let (metadata, _) = read_thrift_metadata::<FileMetaData>(metadata_bytes)?;
```

</details>

<details>
  <summary>Solution</summary>

```rust,ignore
pub fn read_file_metadata(data: Bytes) -> Result<FileMetaData> {
    let metadata_size = data.slice(data.len() - 8..).get_u32_le() as usize;
    let metadata_bytes = data.slice(data.len() - 8 - metadata_size..data.len() - 8);
    let (metadata, _) = read_thrift_metadata::<FileMetaData>(metadata_bytes)?;
    Ok(metadata)
}
```

</details>
