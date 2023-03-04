use std::sync::{Arc, RwLock};
use tonic::{Request, Response, Status};
use crate::api::pb::*;
use crate::core;

pub struct Library {
    library: Arc<RwLock<core::Library>>,
}

impl Library {
    pub fn new(library: core::Library) -> Self {
        Self {
            library: Arc::new(RwLock::new(library))
        }
    }

    pub fn server(self) -> library_server::LibraryServer<Self> {
        library_server::LibraryServer::new(self)
    }
}

#[tonic::async_trait]
impl library_server::Library for Library {
    async fn echo(&self, request: Request<EchoRequest>) -> tonic::Result<Response<EchoResponse>> {
        let response = EchoResponse {
            content: request.into_inner().content,
        };

        Ok(Response::new(EchoResponse {
            content: response.content,
        }))
    }

    async fn list_artists(&self, request: Request<ListArtistsRequest>) -> tonic::Result<Response<ListArtistsResponse>> {
        let artists = self.library.read().unwrap().list_artists();
        Ok(Response::new(ListArtistsResponse {
            artists,
        }))
    }
}