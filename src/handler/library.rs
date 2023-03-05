use tracing::instrument;

use crate::db::Database;
use crate::library;

#[derive(Clone)]
pub struct Library {
    library: library::Library,
    db: Database,
}

impl Library {
    pub fn new(library: library::Library, db: Database) -> Self {
        Self { library, db }
    }

    async fn sync(&self) {}

    #[instrument(skip(self))]
    pub async fn list_artists(&self) -> Vec<String> {
        self.sync().await;
        self.library.list_artists().await
    }
}
