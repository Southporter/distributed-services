use bytes::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub value: Bytes,
    pub offset: usize,
}
