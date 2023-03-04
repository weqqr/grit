#![forbid(unsafe_code)]

mod api;
mod core;

use crate::core::Library;
use anyhow::Result;
use tonic::transport::Server;
use tracing_subscriber::fmt::format::FmtSpan;

#[derive(Debug, serde::Deserialize)]
struct Config {
    listen_address: String,
    library_path: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = std::fs::read_to_string("grit.toml").unwrap();
    let config: Config = toml::from_str(&config).unwrap();

    tracing_subscriber::fmt()
        .with_file(true)
        .with_line_number(true)
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::ENTER)
        .init();

    let addr = "[::]:4444".parse()?;

    let library = Library::new(config.library_path);

    Server::builder()
        .add_service(api::Library::new(library).server())
        .serve(addr)
        .await?;

    Ok(())
}
