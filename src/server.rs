use tonic::{
    transport::{Identity, Server, ServerTlsConfig},
    Request, Response, Status,
};

mod log {
    tonic::include_proto!("log.v1");
}

mod internal;

use crate::internal::store::Store;
use log::logger_server::{Logger, LoggerServer};
use log::{ReadRequest, ReadResponse, WriteRequest, WriteResponse};
use logging::info;
use tokio::sync::RwLock;

#[derive(Debug)]
struct LogServer {
    store: RwLock<Store>,
}

#[tonic::async_trait]
impl Logger for LogServer {
    async fn read(&self, req: Request<ReadRequest>) -> Result<Response<ReadResponse>, Status> {
        let offset = req.into_inner().offset;
        info!("Locking store (read)");
        let store = self.store.read().await;
        let res = store.read(offset).await?;
        Ok(Response::new(ReadResponse {
            value: res.into_iter().collect::<Vec<u8>>(),
        }))
    }
    async fn write(&self, req: Request<WriteRequest>) -> Result<Response<WriteResponse>, Status> {
        let value = req.into_inner().value;
        info!("Locking store (write)");
        let mut store = self.store.write().await;
        let (offset, _read) = store.write(&value).await?;
        Ok(Response::new(WriteResponse { offset }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config_path = std::env::var("CONFIG_PATH")
        .or::<Box<dyn std::error::Error>>(Ok(String::from("/opt/oxyqueue/config")))?;
    let cert = std::fs::read_to_string(format!("{config_path}/server.pem"))?;
    let key = std::fs::read_to_string(format!("{config_path}/server.key"))?;
    let tls_config = ServerTlsConfig::new().identity(Identity::from_pem(&cert, &key));

    let addr = "[::1]:50051".parse()?;
    let log = LogServer {
        store: RwLock::new(Store::new("./test.log").await?),
    };

    pretty_env_logger::init();

    Server::builder()
        .tls_config(tls_config)?
        .add_service(LoggerServer::new(log))
        .serve(addr)
        .await?;

    Ok(())
}
