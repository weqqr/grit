use std::sync::Arc;

use anyhow::Result;
use tracing::{instrument, error, info};

#[derive(Clone)]
pub struct Database {
    client: Arc<tokio_postgres::Client>,
}

impl Database {
    #[instrument(skip(conn))]
    pub async fn connect(conn: &str) -> Result<Self> {
        let (client, conn) = tokio_postgres::connect(conn, tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                error!("connection error: {}", e);
            }
        });

        info!("connected to database");

        Ok(Self {
            client: Arc::new(client),
        })
    }
}
