use tracing::instrument;

use crate::library;

#[derive(Clone)]
pub struct Library {
    library: library::Library,
}

impl Library {
    pub fn new(library: library::Library) -> Self {
        Self { library }
    }

    #[instrument(skip(self))]
    pub async fn list_artists(&self) -> Vec<String> {
        self.library.list_artists().await
    }
}
