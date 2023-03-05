use crate::api::pb::*;
use crate::handler;
use tonic::{Request, Response};

pub struct Library {
    library: handler::Library,
}

impl Library {
    pub fn new(library: handler::Library) -> Self {
        Self {
            library,
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
        Ok(Response::new(ListArtistsResponse {
            artists: self.library.list_artists().await,
        }))
    }
}
