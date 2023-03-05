use anyhow::Result;
use tracing::{instrument, debug, error};

pub struct Database {
    client: tokio_postgres::Client,
}

impl Database {
    #[instrument]
    pub async fn connect(conn: &str) -> Result<Self> {
        let (client, conn) = tokio_postgres::connect(conn, tokio_postgres::NoTls).await?;

        tokio::spawn(async move {
            if let Err(e) = conn.await {
                error!("connection error: {}", e);
            }
        });

        debug!("connected to database");

        Ok(Self {
            client,
        })
    }
}
