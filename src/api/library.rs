use crate::api::pb::*;
use crate::core;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::{Request, Response};

pub struct Library {
    library: Arc<RwLock<core::Library>>,
}

impl Library {
    pub fn new(library: core::Library) -> Self {
        Self {
            library: Arc::new(RwLock::new(library)),
        }
    }

    pub fn server(self) -> library_server::LibraryServer<Self> {
        library_server::LibraryServer::new(self)
    }
}

#[tonic::async_trait]
impl library_server::Library for Library {
    async fn list_artists(
        &self,
        request: Request<ListArtistsRequest>,
    ) -> tonic::Result<Response<ListArtistsResponse>> {
        let library = self.library.read().await;

        Ok(Response::new(ListArtistsResponse {
            artists: library.list_artists().await,
        }))
    }
}
