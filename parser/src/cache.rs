use anyhow::Result;

use std::env;
use std::fs;
use std::path::PathBuf;

const CACHE_DIR: &str = ".sharkalyze_cache";

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

/// Check if the page is already cached.
pub fn is_cached(url: &str) -> Option<PathBuf> {
    let cache_file = ensure_cache_dir().unwrap().join(format!("{}.txt", url));

    if cache_file.exists() {
        Some(cache_file)
    } else {
        None
    }
}
