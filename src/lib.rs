use std::io::Result;
use async_compression::tokio::write::{DeflateDecoder, DeflateEncoder};
use bytes::{BufMut, BytesMut};
use tokio::io::AsyncWriteExt;

async fn compress(data: &[u8]) -> Result<Vec<u8>> {
  let mut encoder = DeflateEncoder::new(Vec::new());

  encoder.write_all(data).await?;
  encoder.shutdown().await?;

  Ok(encoder.into_inner())
}

async fn decompress(data: &[u8]) -> Result<Vec<u8>> {
  let mut decoder = DeflateDecoder::new(Vec::new());

  decoder.write_all(data).await?;
  decoder.shutdown().await?;
  
  Ok(decoder.into_inner())
}

#[derive(Debug)]
pub struct File {
  pub name: String,
  pub content: Vec<u8>
}

impl File {
  pub fn new(name: &str, content: Vec<u8>) -> File {
    File {
      name: name.to_owned(),
      content
    }
  }
}

pub struct Archiver {
  pub stack: Vec<File>,
  pub bytes: BytesMut
}

impl Archiver {
  pub fn new() -> Archiver {
    Archiver {
      stack: Vec::new(),
      bytes: BytesMut::new()
    }
  }

  pub fn put(&mut self, name: &str, content: Vec<u8>) {
    self.stack.push(File::new(name, content));
  }

  async fn write(&mut self, name: &str, content: &[u8]) -> Result<()> {
    let filename_len = name.len() as u16;
    let filename_size = filename_len.to_le_bytes();
    let filename_content = name.as_bytes();

    let content_compressed = compress(&content).await?;
    let content_len = content_compressed.len() as u32;
    let content_size = content_len.to_le_bytes();

    self.bytes.put(&filename_size[..]);
    self.bytes.put(filename_content);

    self.bytes.put(&content_size[..]);
    self.bytes.put(&content_compressed[..]);

    Ok(())
  }

  pub async fn end(&mut self) {
    let files: Vec<File> = self.stack.drain(..).collect();

    for file in files {
      let _ = self.write(&file.name, &file.content).await;
    }
  }
}

pub struct Unarchiver {
  pub files: Vec<File>
}

impl Unarchiver {
  pub fn new() -> Unarchiver {
    Unarchiver {
      files: Vec::new()
    }
  }

  pub async fn read(&mut self, bytes: &mut BytesMut) {
    let size = bytes.len();
    let mut remained = size;

    while remained > 0 {
      if remained < 2 {
        return
      }

      let le_len = bytes.split_to(2);
      let len: usize = u16::from_le_bytes(le_len.as_ref().try_into().unwrap()).into();

      remained -= 2;

      if remained < len {
        return
      }

      let str = bytes.split_to(len);
      let name = String::from_utf8(str.to_vec()).unwrap();

      remained -= len;

      if remained < 4 {
        return
      }

      let le_len = bytes.split_to(4);
      let len = u32::from_le_bytes(le_len.as_ref().try_into().unwrap()) as usize;

      remained -= 4;

      let compressed = bytes.split_to(len);

      if let Ok(content) = decompress(&compressed).await {
        remained -= len;

        self.files.push(File {
          name,
          content
        });
      } else {
        return
      }
    }
  }
}
