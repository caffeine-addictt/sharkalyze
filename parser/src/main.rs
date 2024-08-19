use anyhow::{Context, Result};
use clap::Parser;
// use futures_util::StreamExt;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use url::Url;

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::time::Instant;

mod cache;
mod status;
mod weburl;

fn parse_from_file(path: &str) -> Result<Vec<Result<Url>>> {
    let file = File::open(path).with_context(|| format!("failed to open file: {path}"))?;
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
    let spinner_style = ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg}")?
        .tick_chars("⠁⠂⠄⡀⢀⠠⠐⠈ ");

    for to_fetch in urls.iter().flatten() {
        let task_start = Instant::now();
        let formatter = status::Status::new(&task_start, to_fetch);

        // Add new bar
        // 1 - Check cache
        // 2 - Fetch
        // 3 - parse html / writing to cache
        // 4 - cleanup
        // 5 - waiting
        let prog_bar = multi.add(ProgressBar::new(*status::TERM_WIDTH as u64));
        prog_bar.set_style(spinner_style.clone());
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

        // Http GET
        prog_bar.set_prefix("[2/5]");
        prog_bar.set_message(formatter.format("fetching..."));

        let mut stream = reqwest::get(to_fetch.as_str()).await?;
        while let Some(item) = stream.chunk().await? {
            if args.debug {
                println!("Chunk: {:?}", item);
            }
        }

        // Done
        prog_bar.set_prefix("[5/5]");
        prog_bar.finish_with_message(formatter.format("done, waiting..."));
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
