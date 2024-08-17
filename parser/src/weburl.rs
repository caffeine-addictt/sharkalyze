use anyhow::{Context, Result};

use lazy_static::lazy_static;
use regex::Regex;
use url::Url;

lazy_static! {
    static ref URL_REGEXP: Regex = Regex::new(r"^https?://").unwrap();
}

pub fn parse_url(url: &str) -> Result<Url> {
    // Check if following RegExp
    if !URL_REGEXP.is_match(url) {
        anyhow::bail!("malformed URL: {}", url);
    }

    Url::parse(url).with_context(|| format!("failed to parse URL: {url}"))
}
