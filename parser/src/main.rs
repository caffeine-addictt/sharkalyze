use anyhow::Result;
use clap::Parser;
use futures_util::future::join_all;
use futures_util::StreamExt;
use indicatif::{HumanDuration, MultiProgress, ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use regex::Regex;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use tokio::sync::Semaphore;
use url::{ParseError, Url};

use std::collections::HashSet;
use std::sync::Arc;
use std::time::{Duration, Instant};

mod asyncreq;
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

    // Ensure cache directory exists
    let cache = cache::Cache::new(cache::ensure_cache_dir()?, args.debug);

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
        let formatter = status::Status::new(to_fetch);
        let semaphore = semaphore.clone();

        let cache = cache.clone();
        let client = client.clone();
        let to_fetch = to_fetch.clone();

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

        if let Some(path) = cache.is_cached(&to_fetch) {
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
            // Wait and get lock
            let task_permit = semaphore.acquire().await.unwrap();
            let formatter = status::Status::new(&to_fetch);

            // Http GET
            prog_bar.set_prefix("[2/5]");
            prog_bar.set_message(formatter.format("fetching..."));

            let request = asyncreq::make_req(client.get(to_fetch.as_str())).await?;
            if !request.status().is_success() {
                return Err(anyhow::anyhow!("failed to fetch url"));
            }

            // Open writer
            let mut stream = request.bytes_stream();
            let mut writer = File::create(cache.get_filename(&to_fetch)?).await?;
            let mut buffer: Vec<u8> = Vec::new();

            let mut urls_of_interest: Vec<String> = Vec::new();

            prog_bar.set_prefix("[3/5]");
            prog_bar.set_message(formatter.format("parsing content..."));

            // Write header
            writer.write_all(b"HTML Content:\n").await?;

            while let Some(Ok(chunk)) = stream.next().await {
                // Append to buffer
                buffer = [buffer, chunk.to_vec()].concat();
                if args.debug {
                    println!("{buffer:?}");
                }

                // Consume buffer (HTML as byte)
                // We want to check if the current index is a seperator character,
                // then check if the characters leading up to it match any regex to work on.
                //
                // If yes, we handle as such, less we write to disk
                let mut i = 0;
                while i < buffer.len() {
                    if args.debug {
                        println!("{}", i);
                    }

                    // Handle separators
                    if SEPARATOR.contains(&buffer[i]) {
                        // Only process up to the separator
                        let consumed = String::from_utf8(buffer[0..i].to_vec())?;

                        // Add to URLs
                        if let Some(capture) = weburl::HTML_URL.captures(&consumed) {
                            urls_of_interest.push(capture[1].to_string());
                        }

                        // Account for case where its not at the end of the element
                        // class="a "
                        //         ^
                        if HTML_TO_SKIP_PRE.is_match(&consumed) {
                            if HTML_TO_SKIP.is_match(&consumed) {
                                // We ignore useless HTML stuff like class and style
                                buffer.drain(0..=i);
                                i = 0;
                            }

                            i += 1;
                            continue;
                        }

                        // Write to disk
                        prog_bar.set_message(formatter.format("writing to cache..."));
                        writer.write_all(&buffer[0..=i]).await?;

                        // Remove the processed part from the buffer
                        buffer.drain(0..=i);
                        i = 0;
                        prog_bar.set_message(formatter.format("parsing content..."));
                        continue;
                    }

                    // Handle incomplete line endings (LF/CRLF)
                    if buffer[i] == b'\r' && (i + 1 == buffer.len() || buffer[i + 1] != b'\n') {
                        if i + 1 < buffer.len() {
                            buffer.remove(i);
                        } else {
                            break;
                        }
                    }

                    // Remove repeated spaces
                    if buffer[i] == b' ' && i > 0 && buffer[i - 1] == b' ' {
                        buffer.remove(i - 1);
                        i -= 1;
                    }

                    // Remove spaces around "="
                    if buffer[i] == b'=' {
                        if i + 1 < buffer.len() && buffer[i + 1] == b' ' {
                            buffer.remove(i + 1);
                        }
                        if i > 0 && buffer[i - 1] == b' ' {
                            buffer.remove(i - 1);
                            i -= 1;
                        }
                    }

                    i += 1;
                }
            }

            // Handle urls of interest
            prog_bar.set_prefix("[4/5]");
            prog_bar.set_message(formatter.format("resolving urls..."));
            let mut post_resolved_urls: Vec<String> = vec![];

            for dirty_url in &urls_of_interest {
                prog_bar.set_message(
                    formatter.format(&format!("resolving urls found... [{dirty_url}]")),
                );

                if !dirty_url.ends_with(".js") {
                    post_resolved_urls.push(format!("{dirty_url} - skipped, not a js file"));
                    continue;
                }

                // Relative urls should be resolved to absolute
                let url = if weburl::SAMESITE_URL_REGEXP.is_match(dirty_url) {
                    // handle root level /
                    let new_base: Result<_, ParseError> = if dirty_url.starts_with('/') {
                        let mut new_to_fetch = to_fetch.clone();
                        new_to_fetch.set_path(dirty_url);

                        Ok(new_to_fetch)
                    } else {
                        to_fetch.join(dirty_url)
                    };

                    if new_base.is_err() {
                        post_resolved_urls
                            .push(format!("{dirty_url} - failed to resolve absolute url"));
                        continue;
                    }

                    match Url::parse(new_base.unwrap().as_str()) {
                        Ok(url) => url.to_string(),
                        Err(err) => {
                            post_resolved_urls.push(format!(
                                "{dirty_url} - failed to resolve absolute url [{err}]"
                            ));
                            continue;
                        }
                    }
                } else {
                    dirty_url.to_string()
                };

                match asyncreq::make_req(client.get(&url)).await {
                    Ok(resp) => {
                        if !resp.status().is_success() {
                            post_resolved_urls.push(format!(
                                "{url} - was NOT successful [Code: {}]",
                                resp.status()
                            ));
                            continue;
                        }

                        let text = resp.text().await.unwrap_or(String::from(""));
                        if !text.is_empty() {
                            post_resolved_urls
                                .push(format!("{url} - successfully fetched but returned nothing"));
                            continue;
                        }

                        writer
                            .write_all(format!("\n\n\n\n{url}:\n{text}").as_bytes())
                            .await?;
                        writer.write_all("\n\n\n\n".as_bytes()).await?;
                    }
                    Err(err) => post_resolved_urls
                        .push(format!("{url} - errored while sending initial GET [{err}]")),
                };
            }

            writer
                .write_all(format!("\n\n\n\n{post_resolved_urls:?}").as_bytes())
                .await?;

            // Done
            writer.flush().await?;
            prog_bar.set_prefix("\x1b[32m[5/5]\x1b[0m");
            prog_bar.finish_with_message(formatter.format(&format!(
                "done after parsing {} urls.",
                post_resolved_urls.len(),
            )));

            // Release lock
            drop(task_permit);

            Ok::<_, anyhow::Error>(())
        };
        // Push the future onto our list of futures.
        futures.push(tokio::spawn(future));
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
