use anyhow::{Context, Result};
use clap::Parser;
use url::Url;

use std::fs::File;
use std::io::{BufRead, BufReader};

mod cache;

fn parse_from_file(path: &str) -> Result<Vec<Result<Url>>> {
    let file = File::open(path).with_context(|| format!("failed to open file: {path}"))?;
    Ok(BufReader::new(file)
        .lines()
        .map(|ln| Url::parse(&ln?).context("Failed to parse url"))
        .collect())
}

/// HTML scrapper and parser
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Url or file path
    url_or_path: String,

    /// Whether to run it in debug mode
    #[arg(short, long, default_value_t = false)]
    debug: bool,
}

impl Args {
    /// Parse url or file and return list of urls
    fn get_urls(&self) -> Result<Vec<Result<Url>>> {
        match Url::parse(&self.url_or_path) {
            Ok(url) => Ok(vec![Ok(url)]),
            Err(e) => {
                if self.debug {
                    eprintln!(
                        "failed to parse url due to reason: {e}\n\
                        falling back to reading as file"
                    );
                }
                Ok(parse_from_file(&self.url_or_path)?)
            }
        }
    }
}

fn main() -> Result<()> {
    let args = Args::parse();
    let urls = args.get_urls()?;

    // Ensure cache directory exists
    let cache = cache::Cache::new(cache::ensure_cache_dir()?, args.debug);

    for to_fetch in urls.iter().flatten() {
        if let Some(path) = cache.is_cached(to_fetch) {
            if args.debug {
                println!(
                    "Found in cache for {to_fetch} at {}, skipping...",
                    path.display()
                );
            }
            continue;
        }

        // TODO: Http GET
    }

    println!("{:?}", urls);

    Ok(())
}
