use tonic::{transport::Server, Request, Response, Status};

mod log {
    tonic::include_proto!("log.v1");
}
