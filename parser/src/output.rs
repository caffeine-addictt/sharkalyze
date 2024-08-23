use anyhow::Result;

use std::env;
use std::fs;
use std::path::PathBuf;

const OUTPUT_DIR: &str = "output";

pub struct Output {
    pub file: fs::File,
    pub filepath: PathBuf,
}

#[derive(Clone, Debug)]
pub struct ParserOutput {
    pub pathbuf: PathBuf,
}

impl ParserOutput {
    pub fn new() -> Result<Self> {
        let pathbuf = ensure_output_dir()?;
        Ok(ParserOutput { pathbuf })
    }

    /// Creates an output file and returns its path
    /// based on the timestamp
    pub fn create_output(&self) -> Result<Output> {
        let filename = format!("{}.json", chrono::Utc::now().format("%Y-%m-%d_%H-%M-%S"));
        let filepath = self.pathbuf.join(filename);
        let file = fs::File::create(&filepath)?;

        Ok(Output { filepath, file })
    }
}

/// Ensure output directory
pub fn ensure_output_dir() -> Result<PathBuf> {
    let output_dir = env::current_dir()?.join(OUTPUT_DIR);

    if !output_dir.exists() {
        fs::create_dir(&output_dir)?;
        return Ok(output_dir);
    }

    // Check dir
    if !output_dir.is_dir() {
        return Err(anyhow::anyhow!(
            "{} is not a directory",
            output_dir.display()
        ));
    }

    Ok(output_dir)
}
