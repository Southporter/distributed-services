use tonic::{transport::Server, Request, Response, Status};

mod log {
    tonic::include_proto!("log.v1");
}

use log::logger_server::{Logger, LoggerServer};
use log::{ReadRequest, ReadResponse, WriteRequest, WriteResponse};

#[derive(Debug, Default)]
struct LogServer {}

#[tonic::async_trait]
impl Logger for LogServer {
    async fn read(&self, _req: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        Ok(Response::new(ReadResponse {
            value: b"Unimplemented".to_vec(),
        }))
    }
    async fn write(&self, _req: Request<WriteRequest>) -> Result<Response<WriteResponse>, Status> {
        Ok(Response::new(WriteResponse { offset: 0 }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:50051".parse()?;
    let log = LogServer::default();

    Server::builder()
        .add_service(LoggerServer::new(log))
        .serve(addr)
        .await?;

    Ok(())
}
