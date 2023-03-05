#![forbid(unsafe_code)]

mod api;
mod db;
mod handler;
mod library;

use crate::db::Database;
use crate::library::Library;
use anyhow::Result;
use tonic::transport::Server;
use tonic_web::GrpcWebLayer;
use tower_http::services::ServeDir;
use tracing::{error, info};

#[derive(Debug, serde::Deserialize)]
struct Config {
    api_address: String,
    web_address: String,
    library_path: String,
    db_conn: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = std::fs::read_to_string("grit.toml")?;
    let config = toml::from_str::<Config>(&config)?;

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let api_address = config.api_address.parse()?;
    let web_address = config.web_address.parse()?;

    info!(?api_address, ?web_address);

    let library = Library::new(config.library_path);

    {
        let library = library.clone();
        tokio::spawn(async move {
            library.index().await;
        });
    }

    let db = Database::connect(&config.db_conn).await?;

    let library_handler = handler::Library::new(library, db);

    let grpc_server = Server::builder()
        .accept_http1(true)
        .layer(GrpcWebLayer::new())
        .add_service(api::Library::new(library_handler).server())
        .serve(api_address);

    let router = axum::Router::new().nest_service("/", ServeDir::new("web/build"));

    let http_server = tokio::spawn(async move {
        axum::Server::bind(&web_address)
            .serve(router.into_make_service())
            .await
    });

    tokio::select! {
        r = grpc_server => if let Err(error) = r {
            error!(%error);
        },
        r = http_server => if let Err(error) = r {
            error!(%error);
        },
    };

    Ok(())
}
