#![forbid(unsafe_code)]

mod api;
mod core;

use anyhow::Result;
use tonic::{Request, Response, Status};
use tonic::transport::Server;
use crate::core::Library;

#[derive(Debug, serde::Deserialize)]
struct Config {
    library_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let addr = "[::]:4444".parse()?;

    let config = std::fs::read_to_string("grit.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    let library = Library::new(config.library_path);

    Server::builder()
        .add_service(api::Library::new(library).server())
        .serve(addr)
        .await?;

    Ok(())
}
