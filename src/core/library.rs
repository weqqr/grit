use anyhow::Result;
use std::collections::HashSet;
use std::io::BufReader;
use std::path::{PathBuf, Path};

use tracing::{error, info, instrument};

#[derive(Debug)]
pub struct Library {
    path: PathBuf,

    artists: HashSet<String>,
}

impl Library {
    pub fn new<P: Into<PathBuf>>(p: P) -> Self {
        Self {
            path: p.into(),
            artists: HashSet::new(),
        }
    }

    fn add_file(&mut self, path: &Path) -> Result<()> {
        if let Some(ext) = path.extension() {
            if ext != "flac" {
                return Ok(())
            }
        }

        let file = std::fs::File::open(path)?;
        let reader = BufReader::new(file);
        let flac = claxon::FlacReader::new(reader)?;

        let artist = flac.get_tag("ARTIST").next().unwrap_or("unknown");

        self.artists.insert(artist.to_owned());

        Ok(())
    }

    #[instrument(skip(self))]
    pub fn index(&mut self) {
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

            if let Err(e) = self.add_file(entry.path()) {
                error!(?e, file_name=?entry.file_name());
            }
        }

        info!(file_count, artists=self.artists.len());
    }

    #[instrument(skip(self))]
    pub async fn list_artists(&self) -> Vec<String> {
        self.artists.iter().cloned().collect()
    }
}
