use std::io::{BufRead, BufReader};

use anyhow::{Context, Result};

use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    pub static ref URL_REGEXP: Regex = Regex::new(r"^https?://").unwrap();
    pub static ref SAMESITE_URL_REGEXP: Regex = Regex::new(r#"^/?[^:?#]+"#).unwrap();
    pub static ref HTML_URL: Regex = Regex::new(r#"(?:href|src)\s*=\s*["']([^"']+)["']"#).unwrap();
}

pub fn parse_url(url: &str) -> Result<Url> {
    // Check if following RegExp
    if !URL_REGEXP.is_match(url) {
        anyhow::bail!("malformed URL: {}", url);
    }

    Url::parse(url).with_context(|| format!("failed to parse URL: {url}"))
}

pub fn parse_from_file(path: &str) -> Result<Vec<Result<Url>>> {
    let file = std::fs::File::open(path).with_context(|| format!("failed to open file: {path}"))?;
    Ok(BufReader::new(file)
        .lines()
        .map(|ln| parse_url(&ln?))
        .collect())
}
