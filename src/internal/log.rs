use actix_web::web::Bytes;
use std::sync::Arc;
use tokio::sync::RwLock;

use super::record::Record;

type Data = Arc<Record>;

pub struct Log {
    records: RwLock<Vec<Data>>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            records: Default::default(),
        }
    }

    pub async fn append(&mut self, data: Bytes) -> usize {
        let mut records = self.records.write().await;
        let record = Record {
            value: data,
            offset: records.len(),
        };
        records.push(Arc::new(record));
        return records.len();
    }

    pub async fn read(&mut self, offset: usize) -> Option<Data> {
        let records = self.records.read().await;
        match records.get(offset) {
            Some(rcd) => Some(rcd.clone()),
            None => None,
        }
    }
}
