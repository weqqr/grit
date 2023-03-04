#![forbid(unsafe_code)]

mod api;
mod core;

use crate::core::Library;
use anyhow::Result;
use tonic::transport::Server;
use tracing::info;
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
        .with_max_level(tracing::Level::INFO)
        .with_span_events(FmtSpan::ENTER)
        .init();

    let listen_address = config.listen_address.parse()?;

    info!(?listen_address);

    let mut library = Library::new(config.library_path);
    library.index();

    Server::builder()
        .add_service(api::Library::new(library).server())
        .serve(listen_address)
        .await?;

    Ok(())
}
