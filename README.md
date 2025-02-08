# Primitive Archiver

It structures data in a repeating sequence for each stored file

1. Filename length (2 bytes)
2. Filename bytes — Maximum of 65,535 bytes.
3. Content length (after compression, 4 bytes)
4. Content bytes (after compression) — Maximum of 4,294,967,295 bytes.

This format allows multiple files to be stored sequentially, even with identical filenames.

## Example

```Rust
use primitive_archiver::{Archiver, Unarchiver};

#[tokio::main]
async fn main() {
  let mut archiver = Archiver::new();

  archiver.put("file.txt", Vec::from("Nothing makes sense anymore."));
  archiver.put("some bytes", vec![1, 2, 3, 4, 5]);

  archiver.end().await;

  dbg!(archiver.bytes.clone());

  let mut unarchiver = Unarchiver::new();

  unarchiver.read(&mut archiver.bytes).await;

  dbg!(unarchiver.files);
}
```

- The `put` method (sync) adds file data to an internal buffer.
- The `end` method (async) finalizes the archive by compressing and appending data to the internal BytesMut buffer.
- The Unarchiver reads and extracts stored files asynchronously.

## Future Improvements

- Support for additional compression algorithms
- Support for `Result` instead of silently discarding files

## Dependencies

- [async_compression](https://github.com/Nullus157/async-compression)
- [tokio](https://github.com/tokio-rs/tokio)
- [bytes](https://github.com/tokio-rs/bytes)