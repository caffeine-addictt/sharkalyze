use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::sync::Semaphore;

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

mod asyncreq;
mod output;
mod parser;
mod status;
mod weburl;

lazy_static! {
    static ref SEPARATOR: HashSet<u8> = HashSet::from_iter("\n\t ".as_bytes().to_vec());
    static ref HTML_TO_SKIP: Regex =
        Regex::new(r#"^(class|id|style|data-\w+)\s*=\s*"[^"]*""#).unwrap();
    static ref HTML_TO_SKIP_PRE: Regex =
        Regex::new(r#"^(class|id|style|data-\w+)\s*=\s*".+"#).unwrap();
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
    /// Parse url or file and return list of valid urls
    fn get_urls(&self) -> Result<HashSet<Url>> {
        match weburl::parse_url(&self.url_or_path) {
            Ok(url) => Ok(HashSet::from([url])),
            Err(e) => {
                if self.debug {
                    eprintln!(
                        "failed to parse url due to reason: {e}\n\
                        falling back to reading as file"
                    );
                }
                Ok(weburl::parse_from_file(&self.url_or_path)?
                    .into_iter()
                    .flatten()
                    .collect())
            }
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let urls = args.get_urls()?;
    if urls.is_empty() {
        anyhow::bail!("no valid urls found");
    }


    // Progress bar
    let start = Instant::now();
    let multi = MultiProgress::new();
    let spinner_style =
        ProgressStyle::with_template("{prefix:.bold.dim} {spinner} {wide_msg} {elapsed_precise}")?
            .tick_chars("-\\|/");

    // Create client here to share connection pool
    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(5));
    let mut futures = vec![];

    for to_fetch in &urls {
        let semaphore = semaphore.clone();

        let client = client.clone();

        let prog_bar = multi.add(ProgressBar::new(*status::TERM_WIDTH as u64));
        prog_bar.set_style(spinner_style.clone());
        prog_bar.enable_steady_tick(Duration::from_millis(500));



                }

                }
            }






    if args.quiet {
        multi.clear()?;
    } else {
        println!(
            urls.len(),
        );
    }

    Ok(())
}
