use std::{
    fs,
    path::{Path, PathBuf},
};

use super::Reader;

pub enum SupportedFormat {
    Toml,
}

impl SupportedFormat {
    fn get_extention(&self) -> String {
        match self {
            SupportedFormat::Toml => "toml".to_string(),
        }
    }
}

pub struct FileSystemLoader<P: AsRef<Path>> {
    format: SupportedFormat,
    base_path: P,
}

impl<P: AsRef<Path>> Reader for FileSystemLoader<P> {
    fn read(&self, id: String) -> Result<String, super::ReadError> {
        let path: PathBuf = self
            .base_path
            .as_ref()
            .join(normalize(&id) + &self.format.get_extention());

        fs::read_to_string(path).map_err(|source| super::ReadError::ReadIOError { id, source })
    }
}

fn normalize(id: &str) -> String {
    id.replace(" ", "_")
        .replace(":", "-")
        .replace("/", "_")
        .replace("\\", "_")
        .to_lowercase()
}
