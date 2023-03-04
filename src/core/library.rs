use std::path::PathBuf;

use tracing::instrument;

#[derive(Debug)]
pub struct Library {
    path: PathBuf,
}

impl Library {
    pub fn new<P: Into<PathBuf>>(p: P) -> Self {
        Self { path: p.into() }
    }

    #[instrument(skip(self))]
    pub fn list_artists(&self) -> Vec<String> {
        vec!["test".to_string()]
    }
}
