use anyhow::Result;
use clap::Parser;
use url::Url;

use core::panic;
use std::fs::File;
use std::io::{BufRead, BufReader};

mod cache;

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
    fn get_urls(&self) -> Result<Vec<String>> {
        // Return if is url
        match Url::parse(&self.url_or_path) {
            Ok(url) => Ok(vec![url.to_string()]),
            Err(e) => {
                if self.debug {
                    println!("Error parsing url: {}", e);
                }

                // Treat url as local file path
                let reader = BufReader::new(File::open(&self.url_or_path)?);
                Ok(reader
                    .lines()
                    .map(|s| {
                        Url::parse(&s.expect("Failed to read line"))
                            .expect("Failed to parse url")
                            .to_string()
                    })
                    .collect())
            }
        }
    }
}

fn main() {
    let args = Args::parse();
    let urls = match args.get_urls() {
        Ok(urls) => urls,
        Err(e) => {
            panic!("{}", e);
        }
    };

    // Ensure cache directory exists
    let cache_dir = match cache::ensure_cache_dir() {
        Ok(c) => c,
        Err(e) => panic!("Failed to create cache dir: {}", e),
    };

    println!("{:?}", urls);
    println!("{:?}", cache_dir);
}
