use std::collections::HashSet;

use anyhow::Result;
use futures_util::StreamExt;
use lazy_static::lazy_static;
use regex::Regex;

use crate::{asyncreq, weburl};

use super::{
    overlap,
    vector::{format_bool, format_u8, Vector},
};

lazy_static! {
    // Breakpoints where we check the consumed bytes
    static ref SEPARATOR: HashSet<u8> = HashSet::from_iter("\n\t> ".as_bytes().to_vec());
// Property
    static ref PROPERTY_TO_SKIP_PRE: Regex = Regex::new(r#"<(title|head).*"#).unwrap();

    // Html
    static ref HTML_TO_SKIP_PRE: Regex = Regex::new(r#"<title.*"#).unwrap();
    static ref HTML_TO_SKIP: Regex = Regex::new(r#"<title.*>.*</\s*title\s*>"#).unwrap();
    static ref HTML_TITLE_CONTENT: Regex = Regex::new(r#"<title.*>(.*?)</\s*title\s*>"#).unwrap();

    static ref HTML_HEAD_SELF_CLOSING: Regex = Regex::new(r#"<head.*/>"#).unwrap();
    static ref HTML_HEAD_STARTING: Regex = Regex::new(r#"<head.*"#).unwrap();
    static ref HTML_HEAD_ENDING: Regex = Regex::new(r#"</\s*head.*>"#).unwrap();
}

/// Crawls only the url.
/// Mutates the vector and returns the discovered urls (not crawled)
pub async fn crawl_page(client: &reqwest::Client, vector: &mut Vector) -> Result<Vec<String>> {
    let mut discovered_urls = vec![];

    let req = asyncreq::make_req(client.get(&vector.url)).await?;
    if !req.status().is_success() {
        anyhow::bail!("failed to fetch url");
    }

    // Check headers
    if let Some(val) = req.headers().get("content-type") {
        if let Ok(header) = val.to_str() {
            vector.contenttype_header_contains_text_html =
                format_bool(header.contains("text/html"));
            vector.is_utf8_from_header = format_bool(header.contains("utf-8"));
        }
    }

    // Crawl page
    let mut stream = req.bytes_stream();
    let mut buffer: Vec<u8> = vec![];

    let mut i = 0;
    let mut in_head_section = false;
    while let Some(Ok(chunk)) = stream.next().await {
        buffer.extend_from_slice(&chunk);
        vector.html_length += chunk.len();

        // Consume buffer (HTML as bytes)
        while i < buffer.len() {
            // Handle separators
            if SEPARATOR.contains(&buffer[i]) {
                let consumed = String::from_utf8(buffer[0..i].to_vec())?;

                // If is a url
                if let Some(capture) = weburl::HTML_URL.captures(&consumed) {
                    let potential_url = capture[1].trim();
                    discovered_urls.push(potential_url.to_string());

                    // Account for href
                    if consumed.contains("href") {
                        vector.hyprlinks_count += 1;
                        if potential_url.is_empty() || potential_url == "#" {
                            vector.null_hyprlinks_count += 1;
                        }
                    }
                }

                // If it is a comment
                if consumed.contains("<!--") {
                    vector.html_comments_count += 1;
                }

                // Check for head tag
                if HTML_HEAD_ENDING.is_match(&consumed) {
                    in_head_section = false;
                } else if !in_head_section && HTML_HEAD_STARTING.is_match(&consumed) {
                    in_head_section = !HTML_HEAD_SELF_CLOSING.is_match(&consumed);
                }

                if consumed.contains("<nav") {
                    vector.navbar_present =
                        format_bool(format_u8(vector.navbar_present) || consumed.contains("<nav"));
                }

                if consumed.contains("<footer") {
                    vector.footer_present = format_bool(
                        format_u8(vector.footer_present) || consumed.contains("<footer"),
                    );
                }

                if consumed.contains("<link") {
                    vector.link_tag_count += 1;
                }

                if consumed.contains("<script") {
                    vector.javascript_count += 1;
                    vector.samesite_javascript_count += 1;
                    vector.javascript_reachable_count += 1;
                }

                if HTML_TO_SKIP_PRE.is_match(&consumed) {
                    if HTML_TO_SKIP.is_match(&consumed) {
                        // Handle content within <title> tag
                        if let Some(capture) = HTML_TITLE_CONTENT.captures(&consumed) {
                            vector.title_tag_in_head_section = format_bool(in_head_section);
                            vector.title_tag_and_url_overlap =
                                overlap::calculate_overlap(&capture[1], &vector.url);
                        }

                        buffer.drain(0..=i);
                        i = 0;
                    }

                    i += 1;
                    continue;
                }

                buffer.drain(0..=i);
                i = 0;
                continue;
            }

            // Remove repeated spaces
            if i > 0 && buffer[i] == b' ' && buffer[i - 1] == b' ' {
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

            // Remove spaces after opening tags
            if buffer[i] == b'<' {
                while i < buffer.len() && buffer[i] == b' ' {
                    buffer.remove(i);
                    i -= 1;
                }
            }

            i += 1;
        }
    }

    Ok(discovered_urls)
}
