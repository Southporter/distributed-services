use bytes::{Bytes, BytesMut};
use std::borrow::BorrowMut;
use std::io::SeekFrom;
use std::mem::size_of;
use tokio::fs::{File, OpenOptions};
use tokio::io::{AsyncReadExt, AsyncSeekExt, AsyncWriteExt, BufStream};
use tokio::sync::RwLock;

#[derive(Debug)]
pub struct Store {
    current_offset: u64,
    buffer: RwLock<BufStream<File>>,
}

impl Store {
    pub async fn new(file_path: &str) -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(file_path)
            .await?;
        let metadata = file.metadata().await?;
        let current_offset = metadata.len() as u64;
        let buffer = RwLock::new(BufStream::new(file));
        Ok(Self {
            current_offset,
            buffer,
        })
    }

    pub async fn write(&mut self, b: &[u8]) -> std::io::Result<(u64, u64)> {
        let offset = self.current_offset;
        let mut buffer = self.buffer.write().await;
        // Write len
        buffer.write_u64(b.len().try_into().unwrap()).await?;
        buffer.write_all(b).await?;
        let b_size = b.len() * size_of::<u8>();
        let read = b_size + size_of::<u64>();
        let read = read as u64;
        self.current_offset = offset + read;
        Ok((offset, read))
    }

    pub async fn read(&self, offset: u64) -> std::io::Result<Bytes> {
        println!("Offset: {}", offset);
        let mut lock = self.buffer.write().await;
        lock.flush().await?;
        println!("Flushed");
        lock.seek(SeekFrom::Start(offset)).await?;
        println!("Seeked");

        let size = lock.read_u64().await?;
        println!("Size of record: {}", size);
        let mut tmp_buffer = BytesMut::new();
        tmp_buffer.resize(size.try_into().unwrap(), 0u8);
        println!("Tmp buffer: {:?}", tmp_buffer);
        lock.read_exact(&mut tmp_buffer).await?;
        println!("Tmp buffer after read: {:?}", tmp_buffer);
        Ok(tmp_buffer.into())
    }

    async fn close(&mut self) -> std::io::Result<()> {
        let mut lock = self.buffer.write().await;
        lock.flush().await?;
        lock.shutdown().await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::fs::{self, File};

    #[tokio::test]
    async fn it_writes_to_file() {
        let file_name = "test1.log";
        let f = File::create(file_name)
            .await
            .expect("Could not create test file");

        let store_res = Store::new(file_name).await;
        assert!(store_res.is_ok());
        let mut store = store_res.unwrap();

        let data = b"Hello Test!";
        let write_res = store.write(data).await;
        assert!(write_res.is_ok());
        let (offset, bytes) = write_res.unwrap();
        assert_eq!(offset, 0);
        let total_written = data.len() + 8;
        assert_eq!(bytes, total_written.try_into().unwrap());
        store.close().await.unwrap();

        let f = File::open(file_name).await.expect("File not opened");
        let metadata = f.metadata().await.unwrap();
        assert_eq!(metadata.len(), 8 + 11);
        fs::remove_file(file_name)
            .await
            .expect("Could not remove file");
    }

    #[tokio::test]
    async fn it_reads_from_file() {
        let file_name = "test2.log";
        let mut f = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_name)
            .await
            .expect("Can't open test log for writing");
        f.set_len(0).await.unwrap();
        let test1 = b"test1";
        let test2 = b"test2";
        let test3 = b"test3";
        println!("Test1 len: {}", test1.len());
        f.write_u64(test1.len() as u64).await.unwrap();
        f.write_all(test1).await.unwrap();
        f.write_u64(test2.len() as u64).await.unwrap();
        f.write_all(test2).await.unwrap();
        f.flush().await.unwrap();
        let offset = f.metadata().await.unwrap().len();
        f.write_u64(test3.len() as u64).await.unwrap();
        f.write_all(test3).await.unwrap();
        f.flush().await.unwrap();

        let mut store = Store::new(file_name).await.unwrap();

        let res = store.read(offset).await;
        println!("Res: {:?}", res);
        assert!(res.is_ok());
        let b = res.unwrap();

        assert_eq!(b, &test3[..]);

        store.close().await.unwrap();

        fs::remove_file(file_name)
            .await
            .expect("Could not clean up file");
    }

    #[tokio::test]
    async fn it_reads_what_it_writes() {
        let file_name = "test3.log";

        let mut store = Store::new(file_name).await.unwrap();
        store.write(b"this is test1").await.unwrap();
        store.write(b"this is test2").await.unwrap();
        let target = b"This is the target!";
        let (offset, _) = store.write(target).await.unwrap();
        let target2 = b"This is test 3";
        let (offset2, _) = store.write(target2).await.unwrap();
        store.write(b"this is test4").await.unwrap();

        let result = store.read(offset).await.unwrap();

        assert_eq!(&target[..], result);

        store.write(b"This is test 5").await.unwrap();
        store.write(b"This is test 6").await.unwrap();

        let result = store.read(offset2).await.unwrap();

        assert_eq!(&target2[..], result);

        fs::remove_file(file_name)
            .await
            .expect("Could not clean up file");
    }
}
