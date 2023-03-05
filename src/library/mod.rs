use anyhow::Result;
use tracing::{instrument, error, info};
use std::collections::HashSet;
use std::io::BufReader;
use std::path::{PathBuf, Path};
use std::sync::Arc;

use tokio::sync::RwLock;

#[derive(Clone)]
pub struct Library {
    path: PathBuf,

    artists: Arc<RwLock<HashSet<String>>>,
}

impl Library {
    pub fn new<P: Into<PathBuf>>(path: P) -> Self {
        Self {
            path: path.into(),
            artists: Arc::new(RwLock::new(HashSet::new())),
        }
    }

    async fn add_file(&self, path: &Path) -> Result<()> {
        if let Some(ext) = path.extension() {
            if ext != "flac" {
                return Ok(())
            }
        }

        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let flac = claxon::FlacReader::new(reader)?;

        let artist = flac.get_tag("ARTIST").next().unwrap_or("unknown");

        self.artists.write().await.insert(artist.to_owned());

        Ok(())
    }

    #[instrument(skip(self))]
    pub async fn index(&self) {
        let mut file_count = 0;

        for entry in walkdir::WalkDir::new(&self.path) {
            let entry = match entry {
                Ok(entry) => entry,
                Err(e) => {
                    error!("{}", e);
                    continue;
                }
            };

            if !entry.file_type().is_file() {
                continue;
            }

            file_count += 1;

            if let Err(e) = self.add_file(entry.path()).await {
                error!(?e, file_name=?entry.file_name());
            }
        }

        let artists_count = self.artists.read().await.len();

        info!(file_count, artists_count);
    }

    #[instrument(skip(self))]
    pub async fn list_artists(&self) -> Vec<String> {
        self.artists.read().await.iter().cloned().collect()
    }
}
