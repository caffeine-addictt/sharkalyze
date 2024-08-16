use anyhow::Result;

use std::collections::HashSet;
use std::env;
use std::fs;
use std::path::PathBuf;

const CACHE_DIR: &str = ".sharkalyze_cache";

pub struct Cache {
    pub pathbuf: PathBuf,
    pub cached_files: HashSet<String>,
}

impl Cache {
    pub fn new(pathbuf: PathBuf) -> Self {
        let mut cached_files: HashSet<String> = HashSet::new();
        for entry in pathbuf.read_dir().unwrap() {
            cached_files.insert(entry.unwrap().file_name().into_string().unwrap());
        }

        Cache {
            pathbuf,
            cached_files,
        }
    }

    /// Check if the page is already cached
    /// Returns the file PathBuf if cached
    fn is_cached(&self, url: &str) -> Option<PathBuf> {
        let cached_file = self.pathbuf.join(format!("{}.txt", url));

        if cached_file.exists() {
            Some(cached_file)
        } else {
            None
        }
    }
}

/// Ensure cache directory
pub fn ensure_cache_dir() -> Result<PathBuf> {
    let cache_dir = env::current_dir()?.join(CACHE_DIR);

    if !cache_dir.exists() {
        fs::create_dir(&cache_dir)?;
        return Ok(cache_dir);
    }

    // Check dir
    if !cache_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "{} is not a directory",
            cache_dir.display()
        ));
    }

    Ok(cache_dir)
}
