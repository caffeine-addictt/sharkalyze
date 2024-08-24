use std::{
    collections::HashSet,
    io::{BufRead, BufReader},
};

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

fn parse_from_file(path: &str) -> Result<Vec<Result<Url>>> {
    let file = std::fs::File::open(path).with_context(|| format!("failed to open file: {path}"))?;
    Ok(BufReader::new(file)
        .lines()
        .map(|ln| parse_url(&ln?))
        .collect())
}

pub fn get_urls(url_or_path: &str) -> Result<HashSet<Url>> {
    match parse_url(url_or_path) {
        Ok(url) => Ok(HashSet::from([url])),
        Err(_) => Ok(parse_from_file(url_or_path)?
            .into_iter()
            .flatten()
            .collect()),
    }
}

pub fn calculate_entropy<T: AsRef<[u8]>>(data: T) -> f32 {
    let bytes = data.as_ref();
    let mut entropy = 0.0;
    let mut counts = [0usize; 256];

    for &b in bytes {
        counts[b as usize] += 1;
    }

    for &count in &counts {
        if count == 0 {
            continue;
        }

        let p: f32 = (count as f32) / (bytes.len() as f32);
        entropy -= p * p.log(2.0);
    }

    entropy
}
