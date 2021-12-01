use actix_web::web::Bytes;

#[derive(Clone, Debug, PartialEq)]
pub struct Record {
    pub value: Bytes,
    pub offset: usize,
}
