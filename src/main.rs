use anyhow::Result;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::pb::{EchoRequest, EchoResponse};

pub mod pb {
    tonic::include_proto!("grit");
}

pub struct Service {}

#[tonic::async_trait]
impl pb::grit_server::Grit for Service {
    async fn echo(&self, request: Request<EchoRequest>) -> Result<Response<EchoResponse>, Status> {
        let response = EchoResponse {
            content: request.into_inner().content,
        };

        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::]:4444".parse()?;
    let service = Service{};

    Server::builder()
        .add_service(pb::grit_server::GritServer::new(service))
        .serve(addr)
        .await?;
    Ok(())
}
