use anyhow::{Context, Result};
use clap::Parser;
use futures_util::future::join_all;
use futures_util::StreamExt;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use url::{ParseError, Url};

use std::collections::HashSet;
use std::io::{BufRead, BufReader};
use std::time::{Duration, Instant};

mod cache;
mod status;
mod weburl;

lazy_static! {
    static ref SEPARATOR: HashSet<u8> = HashSet::from_iter("\n\t ".as_bytes().to_vec());
    static ref HTML_TO_SKIP: Regex =
        Regex::new(r#"^(class|id|style|data-\w+)\s*=\s*"[^"]*""#).unwrap();
    static ref HTML_TO_SKIP_PRE: Regex =
        Regex::new(r#"^(class|id|style|data-\w+)\s*=\s*".+"#).unwrap();
}

fn parse_from_file(path: &str) -> Result<Vec<Result<Url>>> {
    let file = std::fs::File::open(path).with_context(|| format!("failed to open file: {path}"))?;
    Ok(BufReader::new(file)
        .lines()
        .map(|ln| weburl::parse_url(&ln?))
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

    /// Whether to run it in quiet mode
    #[arg(long, default_value_t = false)]
    quiet: bool,
}

impl Args {
    /// Parse url or file and return list of urls
    fn get_urls(&self) -> Result<Vec<Result<Url>>> {
        match weburl::parse_url(&self.url_or_path) {
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

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let urls = args.get_urls()?;

    // Ensure cache directory exists
    let cache = cache::Cache::new(cache::ensure_cache_dir()?, args.debug);

    // Progress bar
    let start = Instant::now();
    let multi = MultiProgress::new();
    let spinner_style =
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg} {elapsed_precise}")?
            .tick_chars("-\\|/");

    let mut futures = vec![];

    for to_fetch in urls.iter().flatten() {
        let formatter = status::Status::new(to_fetch);

        let cache = cache.clone();
        let client = client.clone();

        let prog_bar = multi.add(ProgressBar::new(*status::TERM_WIDTH as u64));
        prog_bar.set_style(spinner_style.clone());
        prog_bar.enable_steady_tick(Duration::from_millis(500));

        // Add new bar
        // 1 - Check cache
        // 2 - Fetch
        // 3 - parse html / writing to cache
        // 4 - resolving urls of interest
        // 5 - waiting
        prog_bar.set_prefix("[1/?]");
        prog_bar.set_message(formatter.format("Checking cache..."));

        if let Some(path) = cache.is_cached(to_fetch) {
            if args.debug {
                println!(
                    "Found in cache for {to_fetch} at {}, skipping...",
                    path.display()
                );
            }

            prog_bar.set_prefix("[1/1]");
            prog_bar.finish_with_message(formatter.format("cached, skipping..."));
            continue;
        }

        let future = async move {
            let formatter = status::Status::new(to_fetch);

            }


            Ok::<_, anyhow::Error>(())
        };
        // Push the future onto our list of futures.
        futures.push(future);
    }

    // Wait for all to complete
    let processes = join_all(futures).await;
    for response in processes {
        match response {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }

    if args.quiet {
        multi.clear()?;
    } else {
        println!(
            "{} urls done in {}",
            urls.len(),
            HumanDuration(start.elapsed())
        );
    }

    Ok(())
}
