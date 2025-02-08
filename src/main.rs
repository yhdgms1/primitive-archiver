use primitive_archiver::{Archiver, Unarchiver};

#[tokio::main]
async fn main() {
  let mut archiver = Archiver::new();

  let _ = archiver.put("file.txt", Vec::from("Nothing makes sense anymore."));
  let _ = archiver.put("some bytes", vec![1, 2, 3, 4, 5]);

  archiver.end().await;

  dbg!(archiver.bytes.clone());

  let mut unarchiver = Unarchiver::new();

  unarchiver.read(&mut archiver.bytes).await;

  dbg!(unarchiver.files);
}
