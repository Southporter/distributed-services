use bytes::Bytes;
use tokio::sync::oneshot::Sender;

pub enum Message {
    Append(Bytes, Sender<usize>),
    Read(usize, Sender<Bytes>),
}
