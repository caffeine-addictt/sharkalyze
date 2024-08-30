use anyhow::Result;
use futures_util::future::join_all;
use indicatif::{HumanDuration, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use parser::vector::Vector;
use regex::Regex;
use tokio::sync::{Mutex, Semaphore};

use std::collections::HashSet;
use std::env;
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

#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        anyhow::bail!("usage: {} <url or path>", args[0]);
    }

    let urls = weburl::get_urls(&args[1])?;
    if urls.is_empty() {
        anyhow::bail!("no valid urls found");
    }

    // Ensure output directory exists
    let output = output::ParserOutput::new()?;

    // Progress bar
    let start = Instant::now();
    let total_count = urls.len();
    let error_count = Arc::new(Mutex::new(0));
    let ok_count = Arc::new(Mutex::new(0));

    let progress_global_track = Arc::new(Mutex::new(ProgressBar::new(total_count as u64)));
    let spinner_style = ProgressStyle::default_bar()
        .template(
            "{spinner} {msg:25} [{wide_bar}] {percent}% ({pos}/{len}) {eta} {elapsed_precise}",
        )?
        .progress_chars("#>-")
        .tick_strings(&["-", "\\", "|", "/"]);
    progress_global_track
        .lock()
        .await
        .set_style(spinner_style.clone());

    // Global progress bar
    progress_global_track
        .lock()
        .await
        .enable_steady_tick(Duration::from_millis(500));
    progress_global_track
        .lock()
        .await
        .set_message(status::format_progress_string(
            *ok_count.lock().await,
            *error_count.lock().await,
            total_count,
        ));

    // Create client here to share connection pool
    let client = reqwest::Client::new();
    let semaphore = Arc::new(Semaphore::new(10));
    let mut futures = vec![];

    for to_fetch in &urls {
        let semaphore = semaphore.clone();

        let client = client.clone();
        let to_fetch = to_fetch.clone().to_string();

        let ok_count = Arc::clone(&ok_count);
        let error_count = Arc::clone(&error_count);
        let progress_global_track = Arc::clone(&progress_global_track);

        futures.push(tokio::spawn(async move {
            let _permit = semaphore.acquire().await.unwrap();
            match parser::generate_vector(client, to_fetch.to_string()).await {
                Ok(vector) => {
                    let prog = progress_global_track.lock().await;
                    let mut ok_count = ok_count.lock().await;
                    *ok_count += 1;

                    prog.inc(1);
                    prog.set_message(status::format_progress_string(
                        *ok_count,
                        *error_count.lock().await,
                        total_count,
                    ));
                    Some(vector)
                }
                Err(_) => {
                    let prog = progress_global_track.lock().await;
                    let mut error_count = error_count.lock().await;
                    *error_count += 1;

                    prog.inc(1);
                    prog.set_message(status::format_progress_string(
                        *ok_count.lock().await,
                        *error_count,
                        total_count,
                    ));
                    None
                }
            }
        }));
    }

    // Collect all parallel processed
    let binding = join_all(futures).await;

    // Create new progress bar
    let final_steps_pb = ProgressBar::new((total_count * 4) as u64);
    final_steps_pb.set_style(spinner_style);

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
            final_steps_pb.inc(total_count as u64);
            s
        }
        Err(e) => {
            final_steps_pb.finish_with_message("\x1b[31mFailed to format to json\x1b[0m");
            anyhow::bail!("failed to format output due to: {e}");
        }
    };

    final_steps_pb.set_message("Writing results...");
    let mut out_file = match output.create_output() {
        Ok(o) => {
            final_steps_pb.inc(total_count as u64);
            o
        }
        Err(e) => {
            final_steps_pb.finish_with_message("\x1b[31mFailed to format to json\x1b[0m");
            anyhow::bail!("failed to format output due to: {e}");
        }
    };

    out_file.file.write_all(pretty_string.as_bytes())?;
    out_file.file.sync_all()?;

    final_steps_pb.inc(total_count as u64);
    final_steps_pb.finish_with_message("\x1b[32mDone!\x1b[0m");

    println!(
        "{} urls done in {}.\n{} of {} failed to resolve.",
        total_count,
        HumanDuration(start.elapsed()),
        total_count - vectors.len(),
        total_count
    );

    println!("Written to {}", out_file.filepath.display());

    Ok(())
}
