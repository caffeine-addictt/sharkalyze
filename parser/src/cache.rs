use anyhow::{Context, Result};

use std::env;
use std::fs;
use std::path::PathBuf;
use url::Url;

use filenamify::filenamify;

const CACHE_DIR: &str = ".sharkalyze_cache";

#[derive(Clone, Debug)]
pub struct Cache {
    pub pathbuf: PathBuf,
    debug: bool,
}

impl Cache {
    pub fn new(pathbuf: PathBuf, debug: bool) -> Self {
        Cache { pathbuf, debug }
    }

    /// Check if the page is already cached
    /// Returns the file PathBuf if cached
    pub fn is_cached(&self, url: &Url) -> Option<PathBuf> {
        let cached_file = self.get_filename(url).ok()?;

        if self.debug {
            println!("Checking if {} exists", cached_file.display());
        }

        if cached_file.exists() {
            Some(cached_file)
        } else {
            None
        }
    }

    /// Get filename based on a URL
    pub fn get_filename(&self, url: &Url) -> Result<PathBuf> {
        let filename = format!(
            "{}{}.txt",
            url.host_str()
                .with_context(|| format!("could not resolve host for {url}"))?,
            url.path()
        );

        Ok(self.pathbuf.join(filenamify(filename)))
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
