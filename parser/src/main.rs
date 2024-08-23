use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use parser::vector::Vector;
use regex::Regex;
use tokio::sync::Semaphore;
use url::Url;

use std::collections::HashSet;
use std::io::Write;
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

    // Ensure output directory exists
    let output = output::ParserOutput::new()?;

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
        let to_fetch = to_fetch.clone().to_string();
        let formatter = status::Status::new(to_fetch.clone());

        let prog_bar = multi.add(ProgressBar::new(*status::TERM_WIDTH as u64));
        prog_bar.set_style(spinner_style.clone());
        prog_bar.enable_steady_tick(Duration::from_millis(500));

        prog_bar.set_prefix("[1/5]");
        prog_bar.set_message(formatter.format("Setting things up..."));

        futures.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            match parser::generate_vector(client, to_fetch.to_string(), &prog_bar, &formatter).await
            {
                Ok(vector) => {
                    prog_bar.set_prefix("\x1b[32m[OK]\x1b[0m");
                    prog_bar.finish_with_message(formatter.format("Done!"));

                    Some(vector)
                }
                Err(e) => {
                    prog_bar.set_prefix("\x1b[31m[ERR]\x1b[0m");
                    prog_bar.finish_with_message(
                        formatter.format(&format!("\x1b[31mFailed [{}]\x1b[0m", e)),
                    );

                    None
                }
            }
        }));
    }

    let total_futures = futures.len() as u64;
    let json_stringify_weight = total_futures * 3;
    let io_write_weight = json_stringify_weight + 20;

    // Create new progress bar
    let final_steps_pb = multi.add(ProgressBar::new(
        total_futures + json_stringify_weight + io_write_weight,
    ));
    final_steps_pb.set_style(
        ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )
        .unwrap()
        .progress_chars("##-"),
    );

    // Collect all parallel processed
    let binding = join_all(futures).await;
    let mut vectors: Vec<Vector> = vec![];

    final_steps_pb.set_message("Collecting results...");
    for future in binding {
        if let Ok(Some(vector)) = future {
            vectors.push(vector);
        }
        final_steps_pb.inc(1);
    }

    final_steps_pb.set_message("Formatting results...");
    let pretty_string = match serde_json::to_string_pretty(&vectors) {
        Ok(s) => {
            final_steps_pb.inc(json_stringify_weight);
            s
        }
        Err(e) => {
            final_steps_pb.inc(json_stringify_weight / 2);
            final_steps_pb.finish_with_message("\x1b[31mFailed to format to json\x1b[0m");
            anyhow::bail!("failed to format output due to: {e}");
        }
    };

    final_steps_pb.set_message("Writing results...");
    let mut out_file = match output.create_output() {
        Ok(o) => {
            final_steps_pb.inc(io_write_weight / 2);
            o
        }
        Err(e) => {
            final_steps_pb.inc(io_write_weight / 4);
            final_steps_pb.finish_with_message("\x1b[31mFailed to format to json\x1b[0m");
            anyhow::bail!("failed to format output due to: {e}");
        }
    };

    out_file.file.write_all(pretty_string.as_bytes())?;
    out_file.file.sync_all()?;

    final_steps_pb.inc(io_write_weight / 2);
    final_steps_pb.finish_with_message("\x1b[32mDone!\x1b[0m");

    if args.quiet {
        multi.clear()?;
    } else {
        println!(
            "{} urls done in {}.\n{} of {} failed to resolve.",
            urls.len(),
            HumanDuration(start.elapsed()),
            urls.len() - vectors.len(),
            total_futures
        );
    }

    println!("Written to {}", out_file.filepath.display());

    Ok(())
}
