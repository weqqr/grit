#![forbid(unsafe_code)]

mod api;
mod handler;
mod db;
mod library;

use crate::library::Library;
use crate::db::Database;
use anyhow::Result;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tracing::info;

#[derive(Debug, serde::Deserialize)]
struct Config {
    listen_address: String,
    library_path: String,
    db_conn: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = std::fs::read_to_string("grit.toml")?;
    let config: Config = toml::from_str(&config)?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let listen_address = config.listen_address.parse()?;

    info!(?listen_address);

    let library = Library::new(config.library_path);

    let l = library.clone();
    tokio::spawn(async move {
        l.index().await;
    });

    let db = Database::connect(&config.db_conn).await?;

    let library_handler = handler::Library::new(library);

    Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(api::Library::new(library_handler).server())
        .serve(listen_address)
        .await?;

    Ok(())
}
