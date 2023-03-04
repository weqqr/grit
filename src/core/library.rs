use std::path::Path;

pub struct Library {}

impl Library {
    pub fn new<P: AsRef<Path>>() -> Self {
        Self {}
    }

    pub fn list_artists(&self) -> Vec<String> {
        vec![
            "test".to_string(),
        ]
    }
}
